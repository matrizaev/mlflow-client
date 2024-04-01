import re

import fire
import pandas as pd
import requests
import yaml


def convert_prop_type(x):
    simple_types = {
        "``STRING``": {"type": "string"},
        "``INT64``": {"type": "integer", "format": "int64"},
        "``INT32``": {"type": "integer", "format": "int32"},
        "``BOOL``": {"type": "boolean"},
        "``DOUBLE``": {"type": "number"},
    }

    if "array" in x:
        if ":ref:" in x:
            return {"type": "array", "items": {"$ref": "#/components/schemas/" + x.split(":ref:")[-1].replace("`", "")}}
        else:
            return {"type": "array", "items": simple_types.get(x.split()[-1], None)}
    elif ":ref:" in x:
        return {"$ref": "#/components/schemas/" + x.split(":ref:")[-1].replace("`", "")}
    else:
        return simple_types.get(x, None)


def markdown_table_to_dataframe(lines, part_checkpoint):

    for line in lines:
        if line.startswith(part_checkpoint):
            break
    else:
        return None

    for line in lines:
        if line.startswith("+"):
            break
    else:
        return None

    data = []
    row = 0
    for line in lines:
        if not line.startswith("+") and not line.startswith("|"):
            break
        if line.startswith("+"):
            row += 1
            continue
        values = re.split(r"\s*\|\s*", line)
        values = [val.strip() for val in values][1:-1]
        values.insert(0, row)
        data.append(values)

    if not data:
        return None

    columns = data[0]
    columns[0] = "row"

    df = pd.DataFrame(data[1:], columns=columns).groupby("row").agg(lambda x: " ".join(x).strip())
    if "Type" in df.columns:
        df["Type"] = df["Type"].apply(convert_prop_type)

    if "Description" in df.columns:
        df["required"] = df["Description"].apply(lambda x: "required" in x)

    return df


def main(mlflow_version: str = "v2.11.3", output_file: str = "mlflow.yaml") -> None:
    response = requests.get(
        f"https://raw.githubusercontent.com/mlflow/mlflow/{mlflow_version}/docs/source/rest-api.rst", timeout=10
    )
    response.raise_for_status()

    text_parts = [part.strip() for part in response.text.split(".. _RESTadd:") if part.strip()]
    service_defs = [
        part.strip() for part in text_parts[0].split("\n===========================\n\n\n\n") if part.strip()
    ]
    structure_defs = [part.strip() for part in text_parts[1].split(".. _") if part.strip()]

    services = {}
    structures = {}
    for service_text in service_defs:
        lines = (line for line in service_text.splitlines() if line.strip())
        service = None
        for line in lines:
            if line.startswith(".. _mlflowMlflowService") or line.startswith(".. _mlflowModelRegistryService"):
                service = line.strip().split(".. _")[-1].replace(":", "").lower()
                services[service] = {
                    "endpoint": markdown_table_to_dataframe(lines, "="),
                    "request": markdown_table_to_dataframe(lines, "Request Structure"),
                    "response": markdown_table_to_dataframe(lines, "Response Structure"),
                }

    for structure_text in structure_defs:
        lines = (line for line in structure_text.splitlines() if line.strip())
        structure = None
        for line in lines:
            if line.startswith("mlflow"):
                structure = line.strip().split()[-1].replace(":", "").lower()
                structures[structure] = {
                    "schema": markdown_table_to_dataframe(lines, "-"),
                }

    open_api_spec = {
        "openapi": "3.0.3",
        "info": {"title": "Mlflow", "version": mlflow_version},
        "servers": [{"url": "http://localhost:5000/api"}],
        "paths": {},
        "components": {"schemas": {}},
    }

    for service, service_def in services.items():
        endpoint_path = "/" + service_def["endpoint"]["Endpoint"][1].replace("`", "")
        endpoint_method = service_def["endpoint"]["HTTP Method"][1].replace("`", "").lower()
        request = service_def["request"]
        if endpoint_method == "post":
            required = request[request["required"].isin([True])]["Field Name"].to_list()
            required_params = {"required": required} if required else {}
            request_schema = {
                "requestBody": {
                    "required": bool(required),
                    "description": "Request body",
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    prop["Field Name"]: {**prop["Type"], "description": prop["Description"]}
                                    for prop in request.to_dict(orient="records")
                                },
                                **required_params,
                            },
                        }
                    },
                }
            }
        else:
            request_schema = {
                "parameters": [
                    {
                        "in": "query",
                        "name": prop["Field Name"],
                        "schema": prop["Type"],
                        "required": "required" in prop["Description"],
                        "description": prop["Description"],
                    }
                    for prop in service_def["request"].to_dict(orient="records")
                ]
            }
        if service_def["response"] is not None:
            response_schema = {
                "content": {
                    "application/json": {
                        "schema": {
                            "type": "object",
                            "properties": {
                                prop["Field Name"]: {**prop["Type"], "description": prop["Description"]}
                                for prop in service_def["response"].to_dict(orient="records")
                            },
                        }
                    }
                }
            }
        else:
            response_schema = {}
        open_api_spec["paths"][endpoint_path] = {
            endpoint_method: {**request_schema, "responses": {"200": {"description": "OK", **response_schema}}}
        }

    for structure, structure_def in structures.items():
        if "Field Name" in structure_def["schema"].columns:
            schema_def = {
                "type": "object",
                "properties": {
                    prop["Field Name"]: prop["Type"]
                    for prop in structures[structure]["schema"].to_dict(orient="records")
                },
            }
        else:
            schema_def = {"type": "string", "enum": list(structure_def["schema"]["Name"].unique())}
        open_api_spec["components"]["schemas"][structure] = schema_def

    with open(output_file, "w", encoding="utf-8") as f:
        f.write(yaml.dump(open_api_spec, sort_keys=False))


if __name__ == "__main__":
    fire.Fire(main)

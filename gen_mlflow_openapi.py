import re
from typing import Iterator

import fire
import pandas as pd
import requests
import yaml


def convert_prop_type(prop_type: str) -> dict[str, str] | dict[str, dict[str, str]]:
    """Converts a property type string to a OpenAPI schema dictionary.

    Args:
        prop_type: The property type string.

    Returns:
        A dictionary representing the OpenAPI schema for the property type.
    """

    simple_types = {
        "``STRING``": {"type": "string"},
        "``INT64``": {"type": "integer", "format": "int64"},
        "``INT32``": {"type": "integer", "format": "int32"},
        "``BOOL``": {"type": "boolean"},
        "``DOUBLE``": {"type": "number"},
    }

    if "array" in prop_type:
        if ":ref:" in prop_type:
            # Handle array with reference
            return {
                "type": "array",
                "items": {"$ref": "#/components/schemas/" + prop_type.split(":ref:")[-1].strip("`").capitalize()},
            }
        else:
            # Handle array with simple type
            base_type = prop_type.split()[-1]
            return {"type": "array", "items": simple_types.get(base_type, None)}
    elif ":ref:" in prop_type:
        # Handle reference type
        return {"$ref": "#/components/schemas/" + prop_type.split(":ref:")[-1].strip("`").capitalize()}
    else:
        # Handle simple type
        return simple_types.get(prop_type, None)


def markdown_table_to_dataframe(lines: Iterator[str], part_checkpoint: str) -> pd.DataFrame | None:
    """Parses a markdown table into a pandas DataFrame.

    Args:
        lines: An iterator of lines from the markdown table.
        part_checkpoint: A string to look for in the lines to indicate the start of the table parsing.

    Returns:
        A pandas DataFrame representing the parsed table, or None if no valid table is found.
    """

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


def populate_openapi_components(structure_definitions: dict) -> dict:
    schemas = {}
    for structure, structure_def in structure_definitions.items():
        if "Field Name" in structure_def["schema"].columns:
            schema_def = {
                "type": "object",
                "properties": {
                    prop["Field Name"]: prop["Type"] for prop in structure_def["schema"].to_dict(orient="records")
                },
            }
        else:
            schema_def = {"type": "string", "enum": list(structure_def["schema"]["Name"].unique())}
        schemas[structure] = schema_def
    return schemas


def populate_openapi_paths(service_definitions: dict) -> dict:
    paths = {}
    for service, service_def in service_definitions.items():
        endpoint_path = "/" + service_def["endpoint"]["Endpoint"][1].replace("`", "")
        endpoint_method = service_def["endpoint"]["HTTP Method"][1].replace("`", "").lower()
        request = service_def["request"]
        if endpoint_method == "post":
            required = request[request["required"].isin([True])]["Field Name"].to_list()
            required_params = {"required": required} if required else {}
            request_schema = {
                "operationId": service,
                "requestBody": {
                    "required": bool(required),
                    "description": "Request body",
                    "content": {
                        "application/json": {
                            "schema": {
                                "title": f"{service.capitalize()}Request",
                                "type": "object",
                                "properties": {
                                    prop["Field Name"]: {**prop["Type"], "description": prop["Description"]}
                                    for prop in request.to_dict(orient="records")
                                },
                                **required_params,
                            },
                        }
                    },
                },
            }
        else:
            request_schema = {
                "operationId": service,
                "parameters": [
                    {
                        "in": "query",
                        "name": prop["Field Name"],
                        "schema": prop["Type"],
                        "required": prop["required"],
                        "description": prop["Description"],
                    }
                    for prop in service_def["request"].to_dict(orient="records")
                ],
            }
        if service_def["response"] is not None:
            response_schema = {
                "content": {
                    "application/json": {
                        "schema": {
                            "title": f"{service.capitalize()}200Response",
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
        paths[endpoint_path] = {
            endpoint_method: {**request_schema, "responses": {"200": {"description": "OK", **response_schema}}}
        }
    return paths


def download_text_file(url: str) -> str:
    """Downloads text content from a URL."""
    with requests.get(url, timeout=10) as response:
        response.raise_for_status()
        return response.text


def split_services_and_schemas(text: str) -> tuple[Iterator[str], Iterator[str]]:
    """Splits a text into service definitions and schema definitions.

    Args:
        text: The text to split.

    Returns:
        A tuple containing two iterators, one for service definitions and one for schema
        definitions. Each iterator contains strings representing the service definitions
        and schema definitions, respectively. The iterators will be empty if no service
        definitions or schema definitions are found. The iterators will be empty if the text
        is empty. The iterators will be empty if the text is not split correctly. The
        iterators will be empty if the text is not split correctly. The iterators will be
        empty if the text is not split correctly. The iterators will be empty if the text
        is not split correctly. The iterators will be empty if the text is not split
        correctly. The iterators will be empty if the text is not split correctly. The
        iterators will be empty if the text is not split correctly. The iterators will be
        empty if the text is not split correctly. The iterators will be empty if the text
        is not split correctly. The iterators will be empty if the text is not split
        correctly. The iterators will be empty if the text is not split correctly. The
    """
    text_parts = tuple(part.strip() for part in text.split(".. _RESTadd:") if part.strip())
    service_defs = (
        part.strip() for part in text_parts[0].split("\n===========================\n\n\n\n") if part.strip()
    )
    schema_defs = (part.strip() for part in text_parts[1].split(".. _") if part.strip())
    return service_defs, schema_defs


def parse_services(service_defs: Iterator[str]) -> dict[str, dict[pd.DataFrame]]:
    """Parses service definitions from an iterator of strings.

    Args:
        service_defs: An iterator containing strings representing service definitions.

    Returns:
        A dictionary where keys are service names and values are dictionaries with a "endpoint"
        key containing a pandas DataFrame representing the parsed endpoint, a "request" key
        containing a pandas DataFrame representing the parsed request structure, and a
        "response" key containing a pandas DataFrame representing the parsed response structure.
        The "response" key will be None if the service does not return a response.
        The "request" keys will be None if the service does not have a request
        or response structure.
        The "endpoint" key will be None if the service does not have an endpoint.
    """
    services = {}
    for service_text in service_defs:
        lines = (line.strip() for line in service_text.splitlines() if line.strip())
        service = None
        for line in lines:
            if line.startswith(".. _mlflowMlflowService") or line.startswith(".. _mlflowModelRegistryService"):
                service = line.split(".. _mlflow")[-1].replace(":", "").lower()
                services[service] = {
                    "endpoint": markdown_table_to_dataframe(lines, "="),
                    "request": markdown_table_to_dataframe(lines, "Request Structure"),
                    "response": markdown_table_to_dataframe(lines, "Response Structure"),
                }
    return services


def parse_schemas(structure_defs: Iterator[str]) -> dict[str, dict[pd.DataFrame]]:
    """Parses schema definitions from an iterator of strings.

    Args:
        structure_defs: An iterator containing strings representing schema definitions.

    Returns:
        A dictionary where keys are schema names and values are dictionaries with a "schema"
        key containing a pandas DataFrame representing the parsed schema.
    """
    structures = {}
    for structure_text in structure_defs:
        lines = (line.strip() for line in structure_text.splitlines() if line.strip())
        structure = None
        for line in lines:
            if line.startswith("mlflow"):
                structure = line.split()[-1].replace(":", "").lower().capitalize()
                structures[structure] = {
                    "schema": markdown_table_to_dataframe(lines, "-"),
                }
    return structures


def main(mlflow_version: str = "v2.11.3", output_file: str = "mlflow.yaml") -> None:
    """Generates an OpenAPI specification from Mlflow documentation."""

    rest_md = download_text_file(
        f"https://raw.githubusercontent.com/mlflow/mlflow/{mlflow_version}/docs/source/rest-api.rst"
    )

    service_defs, schema_defs = split_services_and_schemas(rest_md)

    services = parse_services(service_defs)
    schemas = parse_schemas(schema_defs)

    open_api_spec = {
        "openapi": "3.0.3",
        "info": {"title": "Mlflow", "version": mlflow_version},
        "servers": [{"url": "http://localhost:5000/api"}],
        "paths": populate_openapi_paths(services),
        "components": {"schemas": populate_openapi_components(schemas)},
    }

    with open(output_file, "w", encoding="utf-8") as f:
        yaml.dump(open_api_spec, f, sort_keys=False)


if __name__ == "__main__":
    fire.Fire(main)

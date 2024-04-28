from pathlib import Path
import json
from typing import Any


def fix_openapi_spec() -> None:
    openapi_path = Path(__file__).parent.parent / "spec.json"
    openapi: dict[str, Any] = json.loads(openapi_path.read_text())

    security_scheme: dict[str, Any] = {
        "security": [
            {
                "flyAuth": [],
            },
        ],
    }

    component_scheme: dict[str, Any] = {
        "securitySchemes": {
            "flyAuth": {
                "type": "http",
                "scheme": "bearer",
                "bearerFormat": "JWT",
            }
        },
        "responses": {
            "400": {"description": "Check that the parameters were correct."},
            "401": {"description": "The API key used was missing or invalid."},
            "404": {"description": "The resource was not found."},
        },
    }

    openapi.update(security_scheme)
    openapi["components"].update(component_scheme)

    responses: dict[str, Any] = {
        "400": {
            "$ref": "#/components/responses/400",
        },
        "401": {
            "$ref": "#/components/responses/401",
        },
        "404": {
            "$ref": "#/components/responses/404",
        }
    }

    for path in openapi["paths"]:
        for method in openapi["paths"][path]:
            codes_to_add: list[tuple[str, str]] = []
            for path_response in openapi["paths"][path][method]["responses"]:
                print(path_response)
                for code, ref in responses.items():
                    if code not in path_response:
                        print(f"adding code {code} to method {method} in path {path}")
                        codes_to_add.append((code, ref))
            openapi["paths"][path][method]["responses"].update(dict(codes_to_add))

    with open(openapi_path.parent / "fixed_spec.json", "w") as file:
        file.write(json.dumps(openapi, indent=2))


if __name__ == "__main__":
    fix_openapi_spec()

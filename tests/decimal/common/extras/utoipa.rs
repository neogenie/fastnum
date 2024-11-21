macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!($bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!($bits, [< udec $bits >], [<UD $bits>]); }
    };
    ($bits: tt, $dec: ident, $D: ident) => {
        #[allow(dead_code)]
        mod $dec {
            use rstest::*;
            use fastnum::{$D, $dec};
            use utoipa::*;
            use serde_json::json;

            #[derive(ToSchema)]
            struct Pet {
               id: u64,
               name: String,
               age: $D,
            }

            #[rstest(::trace)]
            fn test_utoipa_schema() {
                let schema = Pet::schema();
                let json = serde_json::to_value(schema).unwrap();

                let name = format!("#/components/schemas/{}", stringify!($D));

                assert_eq!(json, json!({
                        "properties": {
                            "age": {"$ref": name},
                            "id": {"format": "int64", "minimum": 0, "type": "integer"},
                            "name": {"type": "string"}
                        },
                        "required": ["id", "name", "age"],
                        "type": "object"
                    })
                );
            }
            
            #[rstest(::trace)]
            fn test_utoipa_api_doc() {

                let name = stringify!($D);
                let ref_ = format!("#/components/schemas/{}", name);

                #[utoipa::path(
                    get,
                    path = "/pets/{id}",
                    responses(
                        (status = 200, description = "Pet found successfully", body = Pet),
                        (status = 404, description = "Pet was not found")
                    ),
                    params(
                        ("id" = u64, Path, description = "Pet database id to get Pet for"),
                    )
                )]
                async fn get_pet_by_id(pet_id: u64) -> Pet {
                    Pet {
                        id: pet_id,
                        age: $dec!(1.2),
                        name: "Lightning".to_string(),
                    }
                }
                
                #[derive(OpenApi)]
                #[openapi(paths(get_pet_by_id))]
                struct ApiDoc;
                
                let json = serde_json::to_value(ApiDoc::openapi()).unwrap();
                
                let expected = json!({
                      "openapi": "3.1.0",
                      "info": {
                        "title": "fastnum",
                        "description": "Fast numbers library",
                        "contact": {
                          "name": "Neo"
                        },
                        "license": {
                          "name": "MIT"
                        },
                        "version": env!("CARGO_PKG_VERSION")
                      },
                      "paths": {
                        "/pets/{id}": {
                          "get": {
                            "tags": [],
                            "operationId": "get_pet_by_id",
                            "parameters": [
                              {
                                "name": "id",
                                "in": "path",
                                "description": "Pet database id to get Pet for",
                                "required": true,
                                "schema": {
                                  "type": "integer",
                                  "format": "int64",
                                  "minimum": 0
                                }
                              }
                            ],
                            "responses": {
                              "200": {
                                "description": "Pet found successfully",
                                "content": {
                                  "application/json": {
                                    "schema": {
                                      "$ref": "#/components/schemas/Pet"
                                    }
                                  }
                                }
                              },
                              "404": {
                                "description": "Pet was not found"
                              }
                            }
                          }
                        }
                      },
                      "components": {
                        "schemas": {
                          "Pet": {
                            "type": "object",
                            "required": [
                              "id",
                              "name",
                              "age"
                            ],
                            "properties": {
                              "age": {
                                "$ref": ref_
                              },
                              "id": {
                                "type": "integer",
                                "format": "int64",
                                "minimum": 0
                              },
                              "name": {
                                "type": "string"
                              }
                            }
                          },
                          name: {
                            "type": "string",
                            "title": name,
                            "format": "double"
                          }
                        }
                      }
                    }
                );
                
                assert_eq!(json, expected);
            }
        }
    };
}

pub(crate) use test_impl;
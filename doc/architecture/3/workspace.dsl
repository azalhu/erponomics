workspace extends ../system-catalog.dsl {
    name "Management"
    description "Manages ERP/User/App settings, licenses, permissions, features, templates, etc."

    model {
        !element emn {
            # Database
            group "DB" {
                emn-db = container "DB" {
                    description "Stores Configurations."
                    technology "SQLite"
                    tags "Database"
                }
                emn-db-api = container "DB API" {
                    description "Web API for accessing Management DB."
                    technology "gRPC + Rust + sqlx"
                    tags "Web API"
                    -> emn-db "Sends SQL Queries" {
                        tags "Container"
                    }
                }
            }
            
            # ERP
            group "ERP" {
                emn-erp-api = container "ERP API" {
                    description "Web API for Managing ERP Connectors."
                    tags "Web API"
                    -> emn-db-api "CRUD Operations" {
                        tags "Container"
                    }
                }
                emn-erp-web = container "ERP Website" {
                    description "Website for Managing ERP Connectors."
                    technology "htmx"
                    tags "Website"
                    -> emn-erp-api "CRUD Operations" {
                        tags "Container"
                    }
                }
            }

            # App
            group "App" {
                emn-app-api = container "App API" {
                    description "Web API for Managing App Connectors."
                    tags "Web API"
                    -> emn-db-api "CRUD Operations" {
                        tags "Container"
                    }
                }
                emn-app-web = container "App Website" {
                    description "Website for Managing App Connectors."
                    technology "htmx"
                    tags "Website"
                    -> emn-app-api "CRUD Operations" {
                        tags "Container"
                    }
                }
            }

            # Organization
            group "Organization" {
                emn-org-api = container "Organization API" {
                    description "Web API for Managing Organizations."
                    tags "Web API"
                    -> emn-db-api "CRUD Operations" {
                        tags "Container"
                    }
                }
                emn-org-web = container "Organization Website" {
                    description "Website for Managing Organizations."
                    technology "htmx"
                    tags "Website"
                    -> emn-org-api "CRUD Operations" {
                        tags "Container"
                    }
                }
            }

            # Domain
            group "Domain" {
                emn-dom-api = container "Domain API" {
                    description "Web API for Managing Domains."
                    tags "Web API"
                    -> emn-db-api "CRUD Operations" {
                        tags "Container"
                    }
                }
                emn-dom-web = container "Domain Website" {
                    description "Website for Managing Domains."
                    technology "htmx"
                    tags "Website"
                    -> emn-dom-api "CRUD Operations" {
                        tags "Container"
                    }
                }
            }

            !adrs decisions
        }
    }

    views {
        systemContext emn "sc-emn" {
            include *
            exclude "relationship.tag!=Software System,Management"
            description "The system context diagram for Management."
        }

        container emn "con-emn" {
            include *
            exclude "relationship.tag!=Container"
            description "The container diagram for Management."
        }
    }

    configuration {
        scope softwareSystem
    }

    !script groovy {
        workspace.trim()
    }
}

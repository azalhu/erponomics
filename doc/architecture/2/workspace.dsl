workspace extends ../system-catalog.dsl {
    name "Manufacturing"
    description "Manages Data and Domain Requirements"

    model {
        !element emf {
            # Database
            group "DB" {
                emf-db = container "DB" {
                    description "Stores Data."
                    technology "SQLite"
                    tags "Database"
                }
                emf-db-api = container "DB API" {
                    description "Web API for accessing Manufacturing DB."
                    technology "gRPC + Rust + sqlx"
                    tags "Web API"
                    -> emf-db "Sends SQL Queries" {
                        tags "Container"
                    }
                }
            }

            # Sync
            group "Sync" {
                emf-sync-db = container "Sync DB" {
                    description "Stores Pending and Ongoing Synchronizations."
                    technology "SQLite"
                    tags "Database"
                }
                emf-sync-queue-req = container "Sync Request Queue" {
                    description "Stores Sync Requests."
                    technology "RabbitMQ"
                    tags "Queue"
                }
                emf-sync-queue-res = container "Sync Result Queue" {
                    description "Stores Sync Results."
                    technology "RabbitMQ"
                    tags "Queue"
                }
                emf-sync = container "Sync Application" {
                    description "Manages Synchronization of Data for Manufacturing."
                    technology "Rust"
                    tags "Serverless Function"
                    -> emf-sync-db "Reads Requests/Results" {
                        tags "Container"
                    }
                    -> emf-sync-queue-req "Publishes Sync Requests" {
                        tags "Container"
                    }
                    -> emf-sync-queue-res "Publishes Sync Results" {
                        tags "Container"
                    }
                }
                emf-sync-api = container "Sync API" {
                    description "Manages Requests for Synchronization of Data for Manufacturing."
                    technology "Rust + Axum"
                    tags "Web API"
                    -> emf-sync-db "Stores Sync Requests/Results" {
                        tags "Container"
                    }
                    -> emf-sync "Initiates/Ends Sync" "via Sync Queue" {
                        tags "Container"
                    }
                }
            }

            # Domain API
            emf-domain-api = container "Domain API" {
                description "Web API for upholding Enterprise Business Rules of Manufacturing."
                technology "GraphQL API + Rust" 
                tags "Web API"
                -> emf-db-api "Queries/Stores Data" {
                    tags "Container"
                }
                -> emf-sync-api "Sends Sync Requests/Results" {
                    tags "Container"
                }
            }

            # Query API#
            emf-query-api = container "Query API" {
                description "Web API for querying data from Manufacturing."
                technology "GraphQL API + Rust" 
                tags "Web API"
                -> emf-db-api "Queries Data" {
                    tags "Container"
                }
            }

            # Gateway
            emf-gateway-api = container "Gateway" {
                description "Gateway for interacting with Manufacturing."
                technology "GraphQL API + Rust" 
                tags "Gateway"
                -> emf-domain-api "Forwards Commands" "GraphQL + HTTP/2" {
                    tags "Container"
                }
                -> emf-query-api "Forwards Queries" "GraphQL + HTTP/2" {
                    tags "Container"
                }
            }

            # Admin
            group "Admin" {
                emf-admin-api = container "Admin API" {
                    description "Web API for interaction with Manufactuing."
                    technology "Axum + Rust"
                    tags "Web API"
                    -> emf-db-api "CRUD operations" {
                        tags "Container"
                    }
                }
                emf-admin-web = container "Admin Website" {
                    description "Admin Website for interacting with Manufacturing."
                    technology "htmx"
                    tags "Website"
                    -> emf-admin-api "CRUD operations" {
                        tags "Container"
                    }
                }
                emf-admin-cli = container "Admin CLI" {
                    description "CLI for interacting with Manufacturing."
                    technology "Rust"
                    tags "CLI"
                    -> emf-admin-api "CRUD operations" {
                        tags "Container"
                    }
                }
            }

            !adrs decisions
        }
    }

    views {
        systemContext emf "sc-emf" {
            include *
            exclude "relationship.tag!=Software System,Manufacturing"
            description "The system context diagram for Manufacturing."
        }

        container emf "con-emf" {
            include *
            exclude "relationship.tag!=Container"
            description "The container diagram for Manufacturing."
        }
    }
    
    configuration {
        scope softwareSystem
    }

    !script groovy {
        workspace.trim()
    }
}

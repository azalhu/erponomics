workspace extends ../system-catalog.dsl {
    name "Manufacturing"
    description "Manages Data and Domain Requirements"

    model {
        properties {
            "structurizr.groupSeparator" "/"
        }

        # Manufacturing
        !element emf {
            emf-db = container "Manufacturing DB" {
                description "Stores Manufacturing Data."
                technology "SQLite"
                tags "Database"
            }
            emf-sync-queue = container "Manufacturing Sync Queue" {
                description "Stores Manufacturing Sync Events."
                technology "RabbitMQ"
                tags "Queue"
                eec -> this "Subscribes to Sync Events" {
                    tags "Container"
                }
            }
            emf-api = container "Manufacturing API" {
                description "Web API for managing Manufacturing."
                technology "Rust + gRPC"
                tags "Web API"
                -> emf-db "Sends SQL Queries" {
                    tags "Container"
                }
                -> emf-sync-queue "Publishes Sync Events" {
                    tags "Container"
                }
                eac -> this "Sends Queries and Commands" {
                    tags "Container"
                }
                eec -> this "Sends Sync Requests" {
                    tags "Container"
                }

                # Entity
                group "Entity" {
                    emf-repository = component "Entity Repository" {
                        description ""
                        technology "Rust + sqlx"
                        tags ""
                        -> emf-db "Sends SQL Queries" {
                            tags "Component"
                        }
                    }
                    emf-command = component "Entity Command Service" {
                        description "Validates and manages State of Entity."
                        technology "Rust"
                        tags ""
                        -> emf-repository "Sends Entity Commands/Queries" {
                            tags "Component"
                        }
                    }
                    emf-query = component "Entity Query Service" {
                        description ""
                        technology "Rust"
                        tags ""
                        -> emf-repository "Sends Entity Queries" {
                            tags "Component"
                        }
                    }
                    emf-sync = component "Entity Sync Service" {
                        description ""
                        technology "Rust"
                        tags ""
                        -> emf-repository "Sends Entity Sync Results/Queries" {
                            tags "Component"
                        }
                        -> emf-sync-queue "Publishes Sync Events" {
                            tags "Component"
                        }
                    }
                    emf-grpc = component "Entity gRPC Service" {
                        description ""
                        technology "gRPC"
                        tags ""
                        -> emf-command "Forwards Entity Commands" {
                            tags "Component"
                        }
                        -> emf-query "Forwards Entity Queries" {
                            tags "Component"
                        }
                        -> emf-sync "Forwards Entity Sync Requests" {
                            tags "Component"
                        }
                        eac -> this "Sends Entity Commands/Queries" {
                            tags "Component"
                        }
                        eec -> this "Sends Entity Sync Requests" {
                            tags "Component"
                        }
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

        component emf-api "com-emf-api" {
            include *
            exclude "relationship.tag!=Component"
            description "The component diagram for Manufacturing API."
        }
    }
    
    configuration {
        scope softwareSystem
    }

    !script groovy {
        workspace.trim()
    }
}

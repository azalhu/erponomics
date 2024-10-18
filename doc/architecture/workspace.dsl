!const ORGANIZATION_NAME "Erponomics"

!const MANUFACTURING_NAME "${ORGANIZATION_NAME} Manufacturing"
!const MANAGEMENT_NAME "${ORGANIZATION_NAME} Management"
!const IDENTITY_PROVISIONING_NAME "${ORGANIZATION_NAME} Identity Provisioning"
!const ERP_CONNECTIVITY_NAME "${ORGANIZATION_NAME} ERP Connectivity"
!const APP_CONNECTIVITY_NAME "${ORGANIZATION_NAME} App Connectivity"

!const SOFTWARE_SYSTEM_TAG "Software System"
!const CONTAINER_TAG "Container"
!const COMPONENT_TAG "Component"

!const EXTERNAL_TAG "External"
!const GATEWAY "Gateway"
!const WEB_APP_TAG "Web App"
!const WEB_API_TAG "Web API"
!const WEBSITE_TAG "Website"
!const DATABASE_TAG "Database"
!const QUEUE_TAG "Queue"
!const CLI_TAG "CLI"

workspace {
    name "${ORGANIZATION_NAME}"
    description "Ergonomic ERP agnostic platform"

    !adrs decisions

    model {
        # SC EMF
        emf = softwareSystem "${MANUFACTURING_NAME}" {
            description "Manages Data and Domain Requirements."
            tags "Core Domain"

            # CONT EMF-DOMAIN
            emf-domain-api = container "Domain API" {
                description "Web API for upholding Enterprise Business Rules of ${MANUFACTURING_NAME}."
                tags "${WEB_API_TAG}"
            }

            # CONT EMF-GATEWAY
            emf-gateway = container "Gateway" {
                description "Gateway for interacting with ${MANUFACTURING_NAME}."
                tags "${GATEWAY}"
            }

            # CONT EMF-QUERY-API
            emf-query-api = container "Query API" {
                description "Web API for querying data from ${MANUFACTURING_NAME}."
                tags "${WEB_API_TAG}"

                # COMP EMF-API-REST
                emf-rest = component "REST Web API" {
                    description "REST API for interacting with ${MANUFACTURING_NAME}."
                    technology "Rust + Axum"
                    tags "${WEB_API_TAG}"
                }

                # COMP EMF-API-GRAPHQL
                emf-graphql = component "GraphQL Web API" {
                    description "GraphQL API for interacting with ${MANUFACTURING_NAME}."
                    technology "Rust + GraphQL"
                    tags "${WEB_API_TAG}"
                }
            }

            # GRP EMF-DB
            group "DB" {
                emf-db = container "DB" {
                    description "Stores Production Orders, Items, Resources, Operation Plans, etc."
                    technology "SQLite"
                    tags "${DATABASE_TAG}"
                }
                emf-db-api = container "DB API" {
                    description "Web API for accessing ${MANUFACTURING_NAME} DB."
                    technology "Rust + gRPC + sqlx"
                    tags "${WEB_API_TAG}"
                }
            }

            # GRP EMF-ADMIN
            group "Admin" {
                emf-admin-browser = container "Admin Website" {
                    description "Admin Website for interacting with ${MANUFACTURING_NAME}."
                    technology "htmx"
                    tags "${WEBSITE_TAG}"
                }
                emf-admin-web = container "Admin Web Application" {
                    description "Admin Web Application for interacting with ${MANUFACTURING_NAME}."
                    technology "Rust + Axum"
                    tags "${WEB_APP_TAG}"
                    emf-admin-api = component "Admin Web API" {
                        description "Admin Web API for interacting with ${MANUFACTURING_NAME}."
                        technology "Rust + Axum"
                        tags "${WEB_API_TAG}"
                    }
                    emf-admin-cli = component "Admin CLI" {
                        description "Admin CLI for interacting with ${MANUFACTURING_NAME}."
                        technology "Rust"
                        tags "${CLI_TAG}"
                    }
                }
            }

            # GRP EMF-SYNC
            group "Sync" {
                emf-sync = container "Sync Application" {
                    description "Manages Synchronization of Data for ${MANUFACTURING_NAME}."
                    technology "Rust"
                    tags "Message Queue"
                }
                emf-sync-api = container "Sync API" {
                    description "Manages Requests for Synchronization of Data for ${MANUFACTURING_NAME}."
                    technology "Rust + Axum"
                    tags "${WEB_API_TAG}"
                }
                emf-sync-db = container "Sync DB" {
                    description "Stores Pending and Ongoing Synchronizations."
                    technology "PostgreSQL"
                    tags "${DATABASE_TAG}"
                }
                emf-sync-queue-req = container "Sync Request Queue" {
                    description "Stores Sync Requests."
                    technology "RabbitMQ"
                    tags "${QUEUE_TAG}"
                }
                emf-sync-queue-res = container "Sync Result Queue" {
                    description "Stores Sync Results."
                    technology "RabbitMQ"
                    tags "${QUEUE_TAG}"
                }
            }
        }
        # SC EAC
        eec = softwareSystem "${ERP_CONNECTIVITY_NAME}" {
            description "Manages Connectivity with ERP Connectors."
            tags "Integration System"
        }
        # SC EAC
        eac = softwareSystem "${APP_CONNECTIVITY_NAME}" {
            description "Manages Connectivity with App Connectors."
            tags "Integration System"
        }
        # SC EMN
        emn = softwareSystem "${MANAGEMENT_NAME}" {
            description "Manages ERP/User/App settings, licenses, permissions, features, templates, etc."
            tags "Integration System"
        }
        # SC EIP
        eip = softwareSystem "${IDENTITY_PROVISIONING_NAME}" {
            description "Manages Authentication/Authorization across Systems."
            tags "Integration System"
        }

        # users
        dom-man = person "Domain Manager" {
            description "Manages Domain."
            tags "Manager"
        }
        erp-man = person "ERP Manager" {
            description "Manages ERP Connector."
            tags "Manager"
        }
        org-man = person "Organization Manager" {
            description "Manages Organization."
            tags "Manager"
        }
        app-man = person "Application Manager" {
            description "Manages App Connector"
            tags "Manager"
        }
        schd-user = person "Schedule User" {
            description "Needs to optimize the execution of Production Orders."
            tags "${EXTERNAL_TAG}" "App User"
        }
        prod-user = person "Produce User" {
            description "Needs to track the status of Production Orders."
            tags "${EXTERNAL_TAG}" "App User"
        }
        insp-user = person "Inspect User" {
            description "Needs to inspect the quality of materials and products of Production Orders."
            tags "${EXTERNAL_TAG}" "App User"
        }

        # external systems/users
        sbo = softwareSystem "SBO Connector" {
            description "Implements ${ORGANIZATION_NAME} ERP API for SAP Business One."
            tags "${EXTERNAL_TAG}" "ERP Connector"
        }
        msbc = softwareSystem "MSBC Connector" {
            description "Implements ${ORGANIZATION_NAME} ERP API for Microsoft Business Central."
            tags "${EXTERNAL_TAG}" "ERP Connector"
        }
        aps = softwareSystem "APS Connector" {
            description "Implements ${ORGANIZATION_NAME} ERP API for Azalhu Production Services."
            tags "${EXTERNAL_TAG}" "ERP Connector"
        }
        odoo = softwareSystem "Odoo Connector" {
            description "Implements ${ORGANIZATION_NAME} ERP API for Odoo."
            tags "${EXTERNAL_TAG}" "ERP Connector"
        }
        schd = softwareSystem "Schedule Connector" {
            description "Implements ${ORGANIZATION_NAME} App API for Schedule."
            tags "${EXTERNAL_TAG}" "App Connector"
        }
        prod = softwareSystem "Produce Connector" {
            description "Implements ${ORGANIZATION_NAME} App API for Produce."
            tags "${EXTERNAL_TAG}" "App Connector"
        }
        insp = softwareSystem "Inspect Connector" {
            description "Implements ${ORGANIZATION_NAME} App API for Inspect."
            tags "${EXTERNAL_TAG}" "App Connector"
        }
        sbo-user = person "SBO User" {
            description "Reads and Updates Data in SBO."
            tags "${EXTERNAL_TAG}" "ERP User"
        }
        msbc-user = person "MSBC User" {
            description "Reads and Updates Data in MSBC."
            tags "${EXTERNAL_TAG}" "ERP User"
        }
        aps-user = person "APS User" {
            description "Reads and Updates Data in APS."
            tags "${EXTERNAL_TAG}" "ERP User"
        }
        odoo-user = person "Odoo User" {
            description "Reads and Updates Data in Odoo."
            tags "${EXTERNAL_TAG}" "ERP User"
        }

        # context relationships to ${MANUFACTURING_NAME}
        emn -> emf "Gets API Metadata" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANUFACTURING_NAME}" "${MANAGEMENT_NAME}"
        }
        eec -> emf "Provides Data from Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANUFACTURING_NAME}" "${ERP_CONNECTIVITY_NAME}"
        }
        eac -> emf "Sends Queries and Commands" "GraphQL/HTTPS" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANUFACTURING_NAME}" "${APP_CONNECTIVITY_NAME}"
        }

        # context relationships to ${MANAGEMENT_NAME}
        eec -> emn "Gets ERP Connectivity for Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}" "${ERP_CONNECTIVITY_NAME}"
        }
        eac -> emn "Verifies App Connectivity to ${ORGANIZATION_NAME}" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}" "${APP_CONNECTIVITY_NAME}"
        }
        app-man -> emn "Sets up Connectivity for App Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}"
        }
        dom-man -> emn "Sets up Domain" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}"
        }
        erp-man -> emn "Sets up Connectivity for ERP Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}"
        }
        org-man -> emn "Sets up Organization" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}"
        }

        # context relationships to ${IDENTITY_PROVISIONING_NAME}
        emf -> eip "Verifies Identity of System" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANUFACTURING_NAME}" "${IDENTITY_PROVISIONING_NAME}"
        }
        emn -> eip "Verifies Identity of System/User" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${MANAGEMENT_NAME}" "${IDENTITY_PROVISIONING_NAME}"
        }
        eec -> eip "Verifies Identity of System/Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${IDENTITY_PROVISIONING_NAME}" "${ERP_CONNECTIVITY_NAME}"
        }
        eac -> eip "Verifies Identity of System/Connector/User" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${IDENTITY_PROVISIONING_NAME}" "${APP_CONNECTIVITY_NAME}"
        }

        # context relationships to ${ERP_CONNECTIVITY_NAME}
        sbo -> eec "Synchronizes Production Orders from SBO" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${ERP_CONNECTIVITY_NAME}"
        }
        msbc -> eec "Synchronizes Production Orders from MSBC" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${ERP_CONNECTIVITY_NAME}"
        }
        aps -> eec "Synchronizes Production Orders from APS" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${ERP_CONNECTIVITY_NAME}"
        }
        odoo -> eec "Synchronizes Production Orders from Odoo" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${ERP_CONNECTIVITY_NAME}"
        }

        # context relationships to ${APP_CONNECTIVITY_NAME}
        schd -> eac "Gets Production Orders and Publishes Schedules" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${APP_CONNECTIVITY_NAME}"
        }
        prod -> eac "Gets Production Orders and Publishes Progress" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${APP_CONNECTIVITY_NAME}"
        }
        insp -> eac "Gets Production Orders and Components and Publishes Inspection Results" {
            tags "${SOFTWARE_SYSTEM_TAG}" "${APP_CONNECTIVITY_NAME}"
        }


        # relationships between people and software systems



        # relationships to/from containers
        eac -> emf-gateway "Sends Queries and Commands" "GraphQL/HTTPS" {
            tags "${CONTAINER_TAG}"
        }
        emf-gateway -> emf-domain-api "Forwards Commands" {
            tags "${CONTAINER_TAG}"
        }
        emf-gateway -> emf-query-api "Forwards Queries" {
            tags "${CONTAINER_TAG}"
        }
        emf-domain-api -> emf-sync-api "Forwards Commands" {
            tags "${CONTAINER_TAG}"
        }
        emf-query-api -> emf-db-api "Queries Data" {
            tags "${CONTAINER_TAG}"
        }

        emf-db-api -> emf-db "Sends SQL Queries/Statements" {
            tags "${CONTAINER_TAG}"
        }
        emf-admin-browser -> emf-admin-web "CRUD operations" {
            tags "${CONTAINER_TAG}"
        }
        emf-admin-web -> emf-db-api "CRUD operations" {
            tags "${CONTAINER_TAG}"
        }

        emf-domain-api -> emf-db-api "Queries/Stores Data" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync-api -> emf-sync-db "Stores Request/Result" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync-api -> emf-sync "Initiates/Ends Sync" "via Sync Queue" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-sync-db "Reads Request/Result" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-sync-queue-req "Publishes Sync Request" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-sync-queue-res "Publishes Sync Result" {
            tags "${CONTAINER_TAG}"
        }

        schd-user -> schd "Schedules Production Orders" "via Schedule" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        prod-user -> prod "Tracks Production Orders" "via Produce" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        insp-user -> insp "Inspects Production Orders" "via Inspect" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }

        eec -> emf-sync-queue-req "Subscribes to Sync Requests" {
            tags "${CONTAINER_TAG}"
        }
        eec -> emf-gateway "Sends Sync Request/Sync Result" {
            tags "${CONTAINER_TAG}"
        }
        eac -> emf-sync-queue-res "Subscribes to Sync Results" {
            tags "${CONTAINER_TAG}"
        }

        emn -> emf-gateway "Gets API Metadata" {
            tags "${CONTAINER_TAG}"
        }

        # relationship to/from components
        eac -> emf-graphql "Sends Queries and Commands" "GraphQL/HTTPS" {
            tags "${COMPONENT_TAG}"
        }
        eac -> emf-rest "Sends Queries and Commands" "JSON/HTTPS" {
            tags "${COMPONENT_TAG}"
        }

        emf-admin-cli -> emf-db-api "CRUD operations" {
            tags "${COMPONENT_TAG}"
        }
        emf-admin-api -> emf-db-api "CRUD operations" {
            tags "${COMPONENT_TAG}"
        }

        # relationships to/from external people and systems
        sbo-user -> sbo "Creates Production Order and Tracks its Process" "via SBO" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        msbc-user -> msbc "Creates Production Order and Tracks its Process" "via MSBC" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        aps-user -> aps "Creates Production Order and Tracks its Process" "via APS" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        odoo-user -> odoo "Creates Production Order and Tracks its Process" "via Odoo" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
    } 

    views {
        systemLandscape "${ORGANIZATION_NAME}" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG}"
            description "The system landscape diagram for ${ORGANIZATION_NAME}."
        }

        systemContext emf "sc-emf" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG},${MANUFACTURING_NAME}"
            description "The system context diagram for ${MANUFACTURING_NAME}."
        }

        container emf "con-emf" {
            include *
            exclude "relationship.tag!=${CONTAINER_TAG}"
            description "The container diagram for ${MANUFACTURING_NAME}."
        }

        component emf-query-api "com-emf-query-api" {
            include *
            exclude "relationship.tag!=${COMPONENT_TAG}"
            description "The component diagram for ${MANUFACTURING_NAME} Web API."
        }

        component emf-admin-browser "com-emf-admin-browser" {
            include *
            exclude "relationship.tag!=${COMPONENT_TAG}"
            description "The component diagram for ${MANUFACTURING_NAME} Admin Website."
        }

        component emf-admin-web "com-emf-admin-web" {
            include *
            exclude "relationship.tag!=${COMPONENT_TAG}"
            description "The component diagram for ${MANUFACTURING_NAME} Admin Web Application."
        }

        systemContext emn "sc-emn" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG},${MANAGEMENT_NAME}"
            autoLayout
            description "The system context diagram for ${ORGANIZATION_NAME} Management."
        }

        container emn "con-emn" {
            include *
            autoLayout
            description "The container diagram for ${ORGANIZATION_NAME} Management."
        }

        systemContext eip "sc-eip" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG},${IDENTITY_PROVISIONING_NAME}"
            autoLayout
            description "The system context diagram for ${ORGANIZATION_NAME} Identity Provisioning."
        }

        container eip "con-eip" {
            include *
            autoLayout
            description "The container diagram for ${ORGANIZATION_NAME} Identity Provisioning."
        }

        systemContext eec "sc-eec" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG},${ERP_CONNECTIVITY_NAME}"
            autoLayout
            description "The system context diagram for ${ERP_CONNECTIVITY_NAME}."
        }

        container eec "con-eec" {
            include *
            autoLayout
            description "The container diagram for ${ERP_CONNECTIVITY_NAME}."
        }

        systemContext eac "sc-eac" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG},${APP_CONNECTIVITY_NAME}"
            autoLayout
            description "The system context diagram for ${ORGANIZATION_NAME} App Connectivity."
        }

        container eac "con-eac" {
            include *
            autoLayout
            description "The container diagram for ${ORGANIZATION_NAME} App Connectivity."
        }

        styles {
            element "Person" {
                background #08427b
                shape Person
            }
            element "${SOFTWARE_SYSTEM_TAG}" {
                background #1168bd
            }
            element "${EXTERNAL_TAG}" {
                background #999999
            }
            element "Container" {
                background #438dd5
            }
            element "${DATABASE_TAG}" {
                shape Cylinder
            }
            element "${QUEUE_TAG}" {
                shape Pipe
            }
            element "Component" {
                background #85bbf0
            }
        }
    }
}

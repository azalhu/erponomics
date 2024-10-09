!const ORGANIZATION_NAME "Erponomics"
!const MANUFACTURING_NAME "${ORGANIZATION_NAME} Manufacturing"
!const ERP_CONNECTIVITY_NAME "${ORGANIZATION_NAME} ERP Connectivity"

!const SOFTWARE_SYSTEM_TAG "Software System"
!const CONTAINER_TAG "Container"
!const COMPONENT_TAG "Component"

!const EXTERNAL_TAG "External"
!const WEB_APP_TAG "Web App"
!const WEB_API_TAG "Web API"
!const WEBSITE_TAG "Website"
!const DATABASE_TAG "Database"
!const QUEUE_TAG "Queue"
!const CLI_TAG "CLI"

workspace {
    name "${ORGANIZATION_NAME}"
    description "Ergonomic ERP agnostic platform"

    model {
        emf = softwareSystem "${MANUFACTURING_NAME}" {
            description "Manages Data and Domain Requirements."
            tags "Core Domain"
            emf-api = container "${MANUFACTURING_NAME} Web API" {
                description "Web API for interacting with ${MANUFACTURING_NAME}."
                tags "${WEB_API_TAG}"
                emf-rest = component "${MANUFACTURING_NAME} REST Web API" {
                    description "REST API for interacting with ${MANUFACTURING_NAME}."
                    technology "Rust + Axum"
                    tags "${WEB_API_TAG}"
                }
                emf-graphql = component "${MANUFACTURING_NAME} GraphQL Web API" {
                    description "GraphQL API for interacting with ${MANUFACTURING_NAME}."
                    technology "Rust + GraphQL"
                    tags "${WEB_API_TAG}"
                }
            }
            group db {
                emf-db = container "${MANUFACTURING_NAME} DB" {
                    description "Stores Production Orders, Items, Resources, Operation Plans, etc."
                    technology "PostgreSQL"
                    tags "${DATABASE_TAG}"
                }
                emf-db-api = container "${MANUFACTURING_NAME} DB API" {
                    description "Web API for accessing ${MANUFACTURING_NAME} DB."
                    technology "Rust + Axum + sqlx"
                    tags "${WEB_API_TAG}"
                }
            }
            group "Admin" {
                emf-admin-browser = container "${MANUFACTURING_NAME} Admin Website" {
                    description "Admin Website for interacting with ${MANUFACTURING_NAME}."
                    technology "htmx"
                    tags "${WEBSITE_TAG}"
                }
                emf-admin-web = container "${MANUFACTURING_NAME} Admin Web Application" {
                    description "Admin Web Application for interacting with ${MANUFACTURING_NAME}."
                    technology "Rust + Axum"
                    tags "${WEB_APP_TAG}"
                    emf-admin-api = component "${MANUFACTURING_NAME} Admin Web API" {
                        description "Admin Web API for interacting with ${MANUFACTURING_NAME}."
                        technology "Rust + Axum"
                        tags "${WEB_API_TAG}"
                    }
                    emf-admin-cli = component "${MANUFACTURING_NAME} Admin CLI" {
                        description "Admin CLI for interacting with ${MANUFACTURING_NAME}."
                        technology "Rust"
                        tags "${CLI_TAG}"
                    }
                }
            }
            group "Sync" {
                emf-sync = container "${MANUFACTURING_NAME} Sync Application" {
                    description "Manages Synchronization of Data for ${MANUFACTURING_NAME}."
                    technology "Rust"
                    tags "Message Queue"
                }
                emf-sync-api = container "${MANUFACTURING_NAME} Sync API" {
                    description "Manages Requests for Synchronization of Data for ${MANUFACTURING_NAME}."
                    technology "Rust + Axum"
                    tags "${WEB_API_TAG}"
                }
                emf-sync-db = container "${MANUFACTURING_NAME} Sync DB" {
                    description "Stores Pending and Ongoing Synchronizations."
                    technology "PostgreSQL"
                    tags "${DATABASE_TAG}"
                }
                emf-sync-queue-req = container "${MANUFACTURING_NAME} Sync Request Queue" {
                    description "Stores Sync Requests."
                    technology "RabbitMQ"
                    tags "${QUEUE_TAG}"
                }
                emf-sync-queue-res = container "${MANUFACTURING_NAME} Sync Result Queue" {
                    description "Stores Sync Results."
                    technology "RabbitMQ"
                    tags "${QUEUE_TAG}"
                }
            }
        }
        eec = softwareSystem "${ERP_CONNECTIVITY_NAME}" {
            description "Manages Connectivity with ERP Connectors."
            tags "Integration System"
        }
        eac = softwareSystem "${ORGANIZATION_NAME} App Connectivity" {
            description "Manages Connectivity with App Connectors."
            tags "Integration System"
        }
        emn = softwareSystem "${ORGANIZATION_NAME} Management" {
            description "Manages ERP/User/App settings, licenses, permissions, features, templates, etc."
            tags "Integration System"
        }
        eip = softwareSystem "${ORGANIZATION_NAME} Identity Provisioning" {
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

        # relationships between people and software systems
        eec -> emf "Provides Data from Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        eac -> emf "Sends Queries and Commands" "GraphQL/HTTPS" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        emn -> emf "Gets API Metadata" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }

        eec -> emn "Gets ERP Connectivity for Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        eac -> emn "Verifies App Connectivity to ${ORGANIZATION_NAME}" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }

        emf -> eip "Verifies Identity of System" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        eec -> eip "Verifies Identity of System/Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        eac -> eip "Verifies Identity of System/Connector/User" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        emn -> eip "Verifies Identity of System/User" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }

        # relationships to/from containers
        eac -> emf-api "Sends Queries and Commands" "GraphQL/HTTPS" {
            tags "${CONTAINER_TAG}"
        }
        emf-api -> emf-db-api "Queries Data" {
            tags "${CONTAINER_TAG}"
        }
        emf-api -> emf-sync-api "Sends Commands" {
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

        emf-sync-api -> emf-sync-db "Stores Request/Result" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync-api -> emf-sync "Initiates/Ends Sync" "via Sync Queue" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-sync-db "Reads Request/Result" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-db-api "Stores Data after Sync" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-sync-queue-req "Publishes Sync Request" {
            tags "${CONTAINER_TAG}"
        }
        emf-sync -> emf-sync-queue-res "Publishes Sync Result" {
            tags "${CONTAINER_TAG}"
        }

        dom-man -> emn "Sets up Domain" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        erp-man -> emn "Sets up Connectivity for ERP Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        org-man -> emn "Sets up Organization" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        app-man -> emn "Sets up Connectivity for App Connector" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        schd-user -> schd "Schedules Production Orders" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        prod-user -> prod "Tracks Production Orders" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        insp-user -> insp "Inspects Production Orders" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }

        eec -> emf-sync-queue-req "Subscribes to Sync Requests" {
            tags "${CONTAINER_TAG}"
        }
        eec -> emf-api "Sends Sync Request/Sync Result" {
            tags "${CONTAINER_TAG}"
        }
        eac -> emf-sync-queue-res "Subscribes to Sync Results" {
            tags "${CONTAINER_TAG}"
        }

        emn -> emf-api "Gets API Metadata" {
            tags "${CONTAINER_TAG}"
        }

        # relationship to/from components
        eac -> emf-graphql "Sends Queries and Commands" "GraphQL/HTTPS" {
            tags "${COMPONENT_TAG}"
        }
        eac -> emf-rest "Sends Queries and Commands" "JSON/HTTPS" {
            tags "${COMPONENT_TAG}"
        }

        emf-admin-cli -> emf-db "CRUD operations" {
            tags "${COMPONENT_TAG}"
        }
        emf-admin-api -> emf-db "CRUD operations" {
            tags "${COMPONENT_TAG}"
        }

        # relationships to/from external people and systems
        sbo -> eec "Synchronizes Production Orders from SBO" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        msbc -> eec "Synchronizes Production Orders from MSBC" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        aps -> eec "Synchronizes Production Orders from APS" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        odoo -> eec "Synchronizes Production Orders from Odoo" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        schd -> eac "Gets Production Orders and Publishes Schedules" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        prod -> eac "Gets Production Orders and Publishes Progress" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        insp -> eac "Gets Production Orders and Components and Publishes Inspection Results" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }

        sbo-user -> sbo "Creates Production Order and Tracks its Process" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        msbc-user -> msbc "Creates Production Order and Tracks its Process" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        aps-user -> aps "Creates Production Order and Tracks its Process" {
            tags "${SOFTWARE_SYSTEM_TAG}"
        }
        odoo-user -> odoo "Creates Production Order and Tracks its Process" {
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
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG}"
            description "The system context diagram for ${MANUFACTURING_NAME}."
        }

        container emf "con-emf" {
            include *
            exclude "relationship.tag!=${CONTAINER_TAG}"
            description "The container diagram for ${MANUFACTURING_NAME}."
        }

        component emf-api "com-emf-api" {
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

        systemContext eec "sc-eec" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG}"
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
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG}"
            autoLayout
            description "The system context diagram for ${ORGANIZATION_NAME} App Connectivity."
        }

        container eac "con-eac" {
            include *
            autoLayout
            description "The container diagram for ${ORGANIZATION_NAME} App Connectivity."
        }

        systemContext emn "sc-emn" {
            include *
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG}"
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
            exclude "relationship.tag!=${SOFTWARE_SYSTEM_TAG}"
            autoLayout
            description "The system context diagram for ${ORGANIZATION_NAME} Identity Provisioning."
        }

        container eip "con-eip" {
            include *
            autoLayout
            description "The container diagram for ${ORGANIZATION_NAME} Identity Provisioning."
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

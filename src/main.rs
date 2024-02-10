use std::net::SocketAddr;
// get SZ4 entiries -> list
// get efbs entries -> get app entires, sort by date, execute old to new
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub links: Vec<Link>,
    pub count: i64,
    pub has_more: bool,
    pub total_result: i64,
    pub tasks: Vec<Task>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub rel: String,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub links: Vec<Link2>,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link2 {
    pub rel: String,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub value: String,
}

async fn returns_json() -> Response {
    let links = vec![
        Link {
            rel: String::from("self"),
            href: String::from("http://test.de/iam/governance/selfservice/api/v1/provtasks"),
        },
        Link {
            rel: String::from("self"),
            href: String::from("http://test.de/iam/governance/selfservice/api/v1/provtasks?offset=1&limit=2&q=Status.Category+eq+Rejected"),
        },
        Link {
            rel: String::from("first"),
            href: String::from("http://test.de/iam/governance/selfservice/api/v1/provtasks?offset=1&limit=2&q=Status.Category+eq+Rejected"),
        },
        Link {
            rel: String::from("next"),
            href: String::from("http://test.de/iam/governance/selfservice/api/v1/provtasks?offset=3&limit=2&q=Status.Category+eq+Rejected"),
        },
    ];

    let links2 = vec![Link2 {
        rel: String::from("linksref2"),
        href: String::from("href2"),
    }];

    let links3 = vec![Link2 {
        rel: String::from("linksref3"),
        href: String::from("href3"),
    }];
    //Process Definition.Tasks.Task Name
    let fields1 = vec![
        Field {
            name: String::from("Process Definition.Tasks.Task Name"),
            value: String::from("Update Number"),
        },
        Field {
            name: String::from("Process Instance.Task Information.Creation Date"),
            value: String::from("2024-01-01T14:51:04Z"),
        },
        Field {
            name: String::from("Process Instance.Task Details.Key"),
            value: String::from("622482"),
        },
        Field {
            name: String::from("APP_INSTANCE_NAME"),
            value: String::from("CAccount"),
        },
    ];

    let fields11 = vec![
        Field {
            name: String::from("Process Definition.Tasks.Task Name"),
            value: String::from("Update Number"),
        },
        Field {
            name: String::from("Process Instance.Task Information.Creation Date"),
            value: String::from("2024-01-02T14:51:04Z"),
        },
        Field {
            name: String::from("Process Instance.Task Details.Key"),
            value: String::from("622483"),
        },
        Field {
            name: String::from("APP_INSTANCE_NAME"),
            value: String::from("AAccount"),
        },
    ];

    let fields12 = vec![
        Field {
            name: String::from("Process Definition.Tasks.Task Name"),
            value: String::from("Create Number"),
        },
        Field {
            name: String::from("Process Instance.Task Information.Creation Date"),
            value: String::from("2024-01-03T14:51:04Z"),
        },
        Field {
            name: String::from("Process Instance.Task Details.Key"),
            value: String::from("622484"),
        },
        Field {
            name: String::from("APP_INSTANCE_NAME"),
            value: String::from("BAccount"),
        },
    ];
    //
    let fields13 = vec![
        Field {
            name: String::from("Process Definition.Tasks.Task Name"),
            value: String::from("Create Number"),
        },
        Field {
            name: String::from("Process Instance.Task Information.Creation Date"),
            value: String::from("2024-01-04T14:51:04Z"),
        },
        Field {
            name: String::from("Process Instance.Task Details.Key"),
            value: String::from("622485"),
        },
        Field {
            name: String::from("APP_INSTANCE_NAME"),
            value: String::from("CAccount"),
        },
    ];

    let fields14 = vec![
        Field {
            name: String::from("Process Definition.Tasks.Task Name"),
            value: String::from("Update Number"),
        },
        Field {
            name: String::from("Process Instance.Task Information.Creation Date"),
            value: String::from("2024-01-05T14:51:04Z"),
        },
        Field {
            name: String::from("Process Instance.Task Details.Key"),
            value: String::from("622486"),
        },
        Field {
            name: String::from("APP_INSTANCE_NAME"),
            value: String::from("AAccount"),
        },
    ];

    let fields2 = vec![
        Field {
            name: String::from("Process Definition.Tasks.Task Name"),
            value: String::from("Update Number"),
        },
        Field {
            name: String::from("Process Instance.Task Information.Creation Date"),
            value: String::from("2023-03-23T14:51:04Z"),
        },
        Field {
            name: String::from("Process Instance.Task Details.Key"),
            value: String::from("622489"),
        },
        Field {
            name: String::from("APP_INSTANCE_NAME"),
            value: String::from("CAccount"),
        },
    ];

    let fields3 = vec![Field {
        name: String::from("Process Instance.Task Information.Retry Task"),
        value: String::from(""),
    }];
    //Objects.Name  Process Instance.Descriptive Data Process Definition.Tasks.Task Name
    let fields4 = vec![Field {
        name: String::from("APP_INSTANCE_NAME"),
        value: String::from(""),
    }];

    let fields5 = vec![Field {
        name: String::from("Objects.Name"),
        value: String::from("APP"),
    }];

    let tasks = vec![
        Task {
            links: links3.clone(),
            fields: fields1,
        },
        Task {
            links: links2.clone(),
            fields: fields11,
        },
        Task {
            links: links3,
            fields: fields12,
        },
        Task {
            links: links2.clone(),
            fields: fields13,
        },
        Task {
            links: links2,
            fields: fields14,
        },
    ];

    let user = Root {
        links,
        count: 2,
        has_more: true,
        total_result: 2,
        tasks,
    };
    Json(user).into_response()
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new().route("/users", get(returns_json));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

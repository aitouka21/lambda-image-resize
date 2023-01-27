use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, QueryInput, QueryOutput};
use serde_dynamo::{from_item, from_items, to_item};

use crate::definition::{Ordering, ReportCursor, ReportItem, ReportReason};

pub struct ReportService {
    client: DynamoDbClient,
    report_table: String,
}

impl Default for ReportService {
    fn default() -> Self {
        let region = Region::default();
        Self {
            client: DynamoDbClient::new(region),
            report_table: std::env::var("REPORT_TABLE")
                .expect("missing REPORT_TABLE in environment variables"),
        }
    }
}

impl ReportService {
    fn report_table(&self) -> String {
        self.report_table.clone()
    }

    pub async fn get_reports(
        &self,
        input: GetReportsInput,
    ) -> Result<GetReportsOutput, Box<dyn std::error::Error>> {
        let query_input = QueryInput {
            table_name: self.report_table(),
            exclusive_start_key: input.cursor.map(|x| to_item(x).unwrap()),
            scan_index_forward: Some(input.ordering.into()),
            ..Default::default()
        };

        let QueryOutput {
            items,
            last_evaluated_key,
            ..
        } = self.client.query(query_input).await?;

        let items = from_items::<_, ReportItem>(items.unwrap())?;

        let token = last_evaluated_key
            .map(from_item::<_, ReportCursor>)
            .map(Result::unwrap)
            .map(ReportCursor::to_token);

        Ok(GetReportsOutput { items, token })
    }
}

#[derive(Default)]
pub struct GetReportsInput {
    pub filter: Option<ReportReason>,
    pub cursor: Option<ReportCursor>,
    pub ordering: Ordering,
}

#[derive(Debug)]
pub struct GetReportsOutput {
    pub items: Vec<ReportItem>,
    pub token: Option<String>,
}

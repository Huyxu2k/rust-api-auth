// use serde::Serialize;
// use sqlx::{MySqlPool,MySqlConnection};

// #[derive(Serialize)]
// pub struct Page<T> {
//     pub message: String,
//     pub data: Vec<T>,
//     pub page_num: i64,
//     pub page_size: i64,
//     pub total_elements: i64,
// }

// impl<T> Page<T> {
//     pub fn new(
//         message: String,
//         data: Vec<T>,
//         page_num: i64,
//         page_size: i64,
//         total_elements: i64,
//     ) -> Page<T> {
//         Page {
//             message: message.to_string(),
//             data,
//             page_num,
//             page_size,
//             total_elements,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct SortedAndPaginated<T> {
//     query: T,
//     sort_by: String,
//     sort_direction: String,
//     page: i64,
//     per_page: i64,
// }
// impl<T> SortedAndPaginated<T> {
//     pub fn per_page(self, per_page: i64) -> Self {
//         SortedAndPaginated { per_page, ..self }
//     }

//     pub fn sort(self, sort_by: String, sort_direction: String) -> Self {
//         SortedAndPaginated {
//             sort_by,
//             sort_direction,
//             ..self
//         }
//     }

//     pub fn load_and_count_items<U>(self, conn: MySqlConnection) -> Result<Page<U>>
//     where 
//     Self: Result<MySqlConnection, (U, i64)>,
//     {
//         let page = self.page;
//         let per_page = self.per_page;
//         let results = self.load::<(U, i64)>(conn)?;
//         let total = results.get(0).map(|x| x.1).unwrap_or(0);
//         let records = results.into_iter().map(|x| x.0).collect();
//         Ok(Page::new("ok".to_string(), records, page, per_page, total))
//     }
// }

// pub trait SortingAndPaging: Sized {
//     fn paginate(self, page: i64) -> SortedAndPaginated<Self>;
// }
// impl<T> SortingAndPaging for T {
//     fn paginate(self, page: i64) -> SortedAndPaginated<Self> {
//         SortedAndPaginated {
//             query: self,
//             sort_by: "".to_string(),
//             sort_direction: "".to_string(),
//             per_page: 20,
//             page,
//         }
//     }
// }

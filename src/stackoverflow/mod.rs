use Solutions;
use reqwest;
const BASE_URL: &str = "http://www.api.stackexchange.com/";

#[derive(Deserialize, Debug,Clone)]
pub struct StackOverflow{
    pub items: Vec<StackOverflowResult>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StackOverflowResult{
    tags: Vec<String>,
    owner: Owner,
    is_answered:bool,
    view_count: i32,
    accepted_answer_id: Option<usize>,
    answer_count: i32,
    score: i32,
    last_activity_date:  Option<i32>,
    creation_date:  i32,
    last_edit_date:  Option<i32>,
    question_id: i32,
    pub link: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Owner{
    reputation: Option<i32>,
    user_id: Option<usize>,
    accept_rate: Option<i32>,
    user_type: String,
    profile_image: Option<String>,
    display_name: String,
    link: Option<String>,
}

impl StackOverflow{
    pub fn new()-> StackOverflow{
        StackOverflow{items: Vec::new()}
    }
}

impl <'a> Solutions<StackOverflow, reqwest::Error> for StackOverflow{
    fn search(&mut self, txt: &str)->Result<StackOverflow, reqwest::Error>{
        //get request to stackexchange webservice
        let mut query = String::from(BASE_URL);
        query.push_str(format!("/2.2/search?order=desc&sort=activity&tagged=rust&intitle={}&site=stackoverflow", txt)
        .as_str());
        let result: StackOverflow = reqwest::get(query.as_str())?
        .json()?;
        Ok(result)
    }

    fn filter_result(&self)->String{
        //TODO
     "filter".to_string() 
    }

    fn preview(&self)->String{
        //TODO
     "preview".to_string() 
    }
}
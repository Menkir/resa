use Solutions;
use reqwest;
use std::cmp;
const BASE_URL: &str = "http://www.api.stackexchange.com/";

#[derive(Deserialize, Debug,Clone)]
pub struct StackOverflow{
    pub items: Vec<StackOverflowResult>,
}

#[derive(Deserialize, Debug,Clone)]
pub struct StackOverflowResult{
    tags: Vec<String>,
    owner: Owner,
    is_answered:bool,
    view_count: i32,
    accepted_answer_id: Option<usize>,
    answer_count: i32,
    pub score: i32,
    last_activity_date:  Option<i32>,
    creation_date:  i32,
    last_edit_date:  Option<i32>,
    question_id: i32,
    pub link: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Clone, Eq)]
struct Owner{
    reputation: Option<i32>,
    user_id: Option<usize>,
    accept_rate: Option<i32>,
    user_type: String,
    profile_image: Option<String>,
    display_name: String,
    link: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct Answers{
    pub items: Vec<Answer>,
    has_more: bool,
    quota_max: i32,
    quota_remaining: i32,
}

#[derive(Deserialize, Debug, Clone, Eq)]
struct Answer{
    owner: Owner,
    is_accepted: bool,
    pub score: i32,
    last_activity_date:  Option<i32>,
    creation_date:  Option<i32>,
    last_edit_date:  Option<i32>,
    answer_id: Option<usize>,
    question_id: i32,
}

impl StackOverflow{
    pub fn new()-> StackOverflow{
        StackOverflow{items: Vec::new()}
    }
}

impl PartialOrd for Answer {
    fn partial_cmp(&self, other: &Answer) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for Answer {
    fn cmp(&self, other: &Answer) -> cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Answer) -> bool {
        self.score == other.score
    }
}

impl PartialEq for Owner {
    fn eq(&self, other: &Owner) -> bool {
        self.accept_rate == other.accept_rate
    }
}

impl <'a> Solutions<StackOverflow, reqwest::Error> for  StackOverflow{
    fn search(&mut self, txt: &str)->Result<StackOverflow, reqwest::Error>{
        //get request to stackexchange webservice
        let mut query = String::from(BASE_URL);
        query.push_str(format!("/2.2/search?order=desc&sort=activity&tagged=rust&intitle={}&site=stackoverflow", txt)
        .as_str());
        let result: StackOverflow = reqwest::get(query.as_str())?
        .json()?;
        Ok(result)
    }

    fn filter(&mut self, amount_results: usize) -> &mut Self{
        let mut answer_ids: Vec<usize> = Vec::new();
        for entry in self.items.clone(){
            if entry.accepted_answer_id.is_some(){
                answer_ids.push(entry.accepted_answer_id.clone().unwrap());
            }
        }

        let mut query = String::from(BASE_URL);
        let mut ids = String::new();
        for id in answer_ids{
            ids.push_str(String::from(format!("{}", id)).as_str());
            ids.push(';');   
        }
        ids.pop(); //remove last ';' 
        query.push_str(format!("/2.2/answers/{}?order=desc&sort=activity&site=stackoverflow", ids)
        .as_str());
        
        let answers_result: Result<Answers, reqwest::Error> = match reqwest::get(query.as_str()){
            Ok(mut r) => r.json(),
            Err(e) => panic!("Query failed {:?}", e)
        };

        let mut answers = answers_result.unwrap().items;
        answers.sort();
        answers = answers.into_iter().take(3).collect();
        let mut items: Vec<StackOverflowResult> = Vec::new();
        for answer in answers{
            self.clone().items.into_iter()
            .for_each(|entry| if entry.accepted_answer_id == answer.answer_id{
                items.push(entry);
            });
        }
        self.items = items;
        self
    }

    fn preview(&self)->String{
        //TODO
     "preview".to_string() 
    }
}
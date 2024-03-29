use dssim::Dssim;
use image::{DynamicImage, GenericImageView};
use itertools::{multizip, Itertools};
use regex::Regex;
use rgb::RGBA8;
use scraper::{ElementRef, Html, Selector};
use std::fs;
use std::path::PathBuf;

use crate::domain::aggregator::questions::Questions;
use crate::domain::aggregator::questions_parser::QuestionsParser;
use crate::domain::entity::question::Question;
use crate::domain::value_object::answer::Answer;

#[derive(Debug)]
pub struct HtmlQuestionsParser {
    path: PathBuf,
}

impl QuestionsParser for HtmlQuestionsParser {
    fn parse(&self) -> Result<Questions, Box<dyn std::error::Error>> {
        let document = fs::read_to_string(self.path.clone().as_path()).unwrap();
        let html = Html::parse_fragment(&document);

        let questions = self.parse_questions(&html);
        let answers = self.parse_answers(&html);

        let score_tuple = self.parse_score(&html);
        let score_values = score_tuple
            .iter()
            .map(|(score, _)| score.clone())
            .collect::<Vec<_>>();
        let max_score_values = score_tuple
            .into_iter()
            .map(|(_, max)| max)
            .collect::<Vec<_>>();

        let max_questions_count = vec![questions.len(), score_values.len(), answers.len()]
            .into_iter()
            .max()
            .unwrap();

        let questions_diff = max_questions_count - questions.len();
        let score_values_diff = max_questions_count - score_values.len();
        let answers_diff = max_questions_count - answers.len();

        let questions_list = multizip((
            [questions, vec!["".to_string(); questions_diff]].concat(),
            [score_values, vec!["".to_string(); score_values_diff]].concat(),
            [max_score_values, vec!["".to_string(); score_values_diff]].concat(),
            [answers, vec![vec![Answer::None]; answers_diff]].concat(),
        ))
        .map(|(question, score, max, answers)| Question::new(question, score, max, answers))
        .collect::<Vec<_>>();

        Ok(Questions::from(questions_list))
    }
}

impl HtmlQuestionsParser {
    fn parse_score(&self, html: &Html) -> Vec<(String, String)> {
        let score_candidate_selector = Selector::parse(".s3+.s3").unwrap();
        let re = Regex::new(r"[\w\s\W]*(?P<score>[\d|,]{4})[\w\s\W]*(?P<max>[\d|,]{4})").unwrap();

        html.select(&score_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .iter()
            .map(|score_line| {
                re.captures(score_line.as_str())
                    .map(|cap| (cap.name("score"), cap.name("max")))
            })
            .map(|mayby_score| mayby_score.expect("Can't parse score tuple"))
            .map(|(one, two)| {
                (
                    one.expect("Can't parse score").as_str().to_string(),
                    two.expect("Can't parse max score").as_str().to_string(),
                )
            })
            .collect::<Vec<_>>()
    }

    fn parse_questions(&self, html: &Html) -> Vec<String> {
        let question_candidate_selector =
            Selector::parse(r#".s3+p+p+p[style*="padding-top: 5pt;padding-left: 19pt;"]"#).unwrap();

        let questions = html
            .select(&question_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .collect();

        questions
    }

    fn parse_answers(&self, html: &Html) -> Vec<Vec<Answer>> {
        let answer_candidate_selector = Selector::parse(".s4+table+span.p").unwrap();
        let re = Regex::new(r"^[\w].\s(?P<answer>[[\w\W]+]+)").unwrap();

        let answers_with_statuses = html
            .select(&answer_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .zip(self.get_answers_selection(html))
            .group_by(|(elem, _)| elem.starts_with("a. "))
            .into_iter()
            .map(|(_, group)| group.collect::<Vec<_>>())
            .chunks(2)
            .into_iter()
            .map(|chunks| chunks.flatten().collect::<Vec<_>>())
            .map(|answers_with_statuses| {
                answers_with_statuses
                    .iter()
                    .map(|(answer, status)| {
                        (
                            re.captures(answer.as_str()).and_then(|c| c.name("answer")),
                            status,
                        )
                    })
                    .map(|(answer, &status)| {
                        Answer::from((answer.unwrap().as_str().to_string(), status))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        answers_with_statuses
    }

    fn parse_answers_images(&self, html: &Html) -> Vec<DynamicImage> {
        let answer_candidate_selector = Selector::parse(".s4+table+span.p").unwrap();
        let image_selector = Selector::parse("img").unwrap();
        let re = Regex::new(r"^[\w\W]*[,](?P<image>[\w\W]*)$").unwrap();

        let answers_images = html
            .select(&answer_candidate_selector)
            .map(|el| {
                el.prev_siblings()
                    .into_iter()
                    .nth(1)
                    .expect("Can't parse radio or checkbox outer container")
            })
            .map(|el| ElementRef::wrap(el).expect("Can't get radio/checkbox element"))
            .map(|el| {
                el.select(&image_selector)
                    .next()
                    .expect("Can't parse img selector")
                    .value()
                    .attr(r"src")
                    .expect("Can't get src from image node")
            })
            .map(|base64_src| {
                re.captures(base64_src)
                    .and_then(|c| c.name("image"))
                    .expect("Can't find base64 regex pattern in image src")
                    .as_str()
                    .to_string()
            })
            .map(|el| base64::decode(el).expect("Can't decode base64 from image src"))
            .map(|el| image::load_from_memory(&el[..]).expect("Can't load image from buffer"))
            .collect::<Vec<_>>();

        answers_images
    }

    fn to_rgba(&self, rgba_image: &DynamicImage) -> Vec<rgb::RGBA8> {
        rgba_image
            .pixels()
            .map(|(_, _, rgba)| rgba.0)
            .map(|vec_rgba| RGBA8::new(vec_rgba[0], vec_rgba[1], vec_rgba[2], vec_rgba[3]))
            .collect::<Vec<_>>()
    }

    fn get_answers_selection(&self, html: &Html) -> Vec<bool> {
        let radio_active_original =
            image::open("./target/button-radio-active.png").expect("Can't open active radio image");
        let radio_rgba_vec = self.to_rgba(&radio_active_original);
        let radio_active_image = Dssim::new()
            .create_image_rgba(
                &radio_rgba_vec[..],
                radio_active_original.width() as usize,
                radio_active_original.height() as usize,
            )
            .expect("Can't create dssim image for radio");

        let checkbox_active_original = image::open("./target/button-checkbox-active.png")
            .expect("Can't open active radio image");
        let checkbox_rgba_vec = self.to_rgba(&checkbox_active_original);
        let checkbox_active_image = Dssim::new()
            .create_image_rgba(
                &checkbox_rgba_vec[..],
                radio_active_original.width() as usize,
                radio_active_original.height() as usize,
            )
            .expect("Can't create dssim image for checkbox");

        self.parse_answers_images(html)
            .into_iter()
            .map(|image| (self.to_rgba(&image), image.width(), image.height()))
            .map(|(image, width, height)| {
                Dssim::new()
                    .create_image_rgba(&image[..], width as usize, height as usize)
                    .expect("Can't create dssim image for answer image")
            })
            .map(|image| {
                (
                    Dssim::new().compare(&radio_active_image, &image).0 < 0.001,
                    Dssim::new().compare(&checkbox_active_image, &image).0 < 0.001,
                )
            })
            .map(|(is_radio_checked, is_checkbox_checked)| is_radio_checked || is_checkbox_checked)
            .collect::<Vec<_>>()
    }
}

impl From<PathBuf> for HtmlQuestionsParser {
    fn from(value: PathBuf) -> HtmlQuestionsParser {
        Self { path: value }
    }
}

use std::{collections::HashMap, fs::File, io::Write, sync::Arc};

use futures::{TryStreamExt, stream::FuturesUnordered};
use rusaint::{
    RusaintError, USaintSession,
    application::{
        USaintClientBuilder,
        course_schedule::{
            CourseScheduleApplication,
            model::{Lecture, LectureCategory},
        },
    },
    model::SemesterType,
};

mod types;

#[tokio::main]
async fn main() -> Result<(), Box<RusaintError>> {
    // for year in 2024..=2025 {
    //     for semester in vec![SemesterType::One, SemesterType::Summer, SemesterType::Two, SemesterType::Winter] {
    //         println!("start {} {}", year, semester);
    //         save_all_lectures(year, semester).await?;
    //     }
    // }
    save_all_lectures(2025, SemesterType::One).await?;

    // multi_major 배열 추가

    // 타전공 인정과목에서 모든 학과에 대한 과목 가져오기
    // 이미 모든 강의에 대한 가져왔기 때문에 어떤 학과에 대한 타전공 인정인지만 파악하면 됨

    // 후처리(해야하나?)

    // DB화 하기

    Ok(())
}

fn semester_to_code(semester: SemesterType) -> &'static str {
    match semester {
        SemesterType::One => "090",
        SemesterType::Summer => "091",
        SemesterType::Two => "092",
        SemesterType::Winter => "093",
    }
}

async fn save_all_lectures(year: u32, semester: SemesterType) -> Result<(), Box<RusaintError>> {
    let session = Arc::new(USaintSession::anonymous());

    // 일단 0부터 9까지 검색한거 모으고 code를 기준으로 중복 제거
    let lectures2: Vec<Vec<Lecture>> = (1..=1)
        .map(|i| find_by_lecture(session.clone(), year, semester, i.to_string(), i))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    let mut map: HashMap<String, Lecture> = HashMap::new();
    for lectures in lectures2 {
        for lec in lectures {
            if map.contains_key(lec.code()) {
                println!(
                    "[warn: duplicate lecture]\norigin {:#?}\nupcoming {:#?}",
                    map.get(lec.code()),
                    lec
                );
                continue;
            }
            map.insert(lec.code().to_string(), lec);
        }
    }

    let lectures = map.values().collect::<Vec<&Lecture>>();
    let json =
        serde_json::to_string_pretty(&lectures).expect("Failed to serialize lectures to JSON");
    std::fs::create_dir_all("../assets").expect("Failed to create dir");
    let mut file = File::create(format!("../assets/lectures/{}_{}.json", year, semester_to_code(semester)))
        .expect("Failed to create json file");
    file.write_all(json.as_bytes())
        .expect("Failed to write file");

    Ok(())
}

async fn find_by_lecture(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    keyword: String,
    delay: u32,
) -> Result<Vec<Lecture>, RusaintError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(delay as u64)).await;
    println!("search '{}' in {} {}", keyword, year, semester);
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await?;
    let category = LectureCategory::find_by_lecture(&keyword);
    let lectures = app.find_lectures(year, semester, &category).await?;
    Ok(lectures.collect())
}

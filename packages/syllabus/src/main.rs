use std::{collections::HashMap, fs::File, io::Write, sync::Arc};

use futures::{
    TryStreamExt,
    stream::FuturesUnordered,
};
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
    //         save_all_lectures(year, semester).await?;
    //     }
    // }
    let lecture_map = save_all_lectures(2025, SemesterType::One).await?;
    // collect_recognized_other_major(2025, SemesterType::One).await?;

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

async fn save_all_lectures(year: u32, semester: SemesterType) -> Result<HashMap<String, Lecture>, Box<RusaintError>> {
    println!("save all lectures: {} {}", year, semester);

    std::fs::create_dir_all("./assets/lectures").expect("Failed to create dir");
    let filename = format!( "./assets/lectures/{}_{}.json", year, semester_to_code(semester));

    let mut lecture_map: HashMap<String, Lecture> = HashMap::new();
    if let Ok(s) = std::fs::read_to_string(&filename) {
        if let Ok(exist) = serde_json::from_str::<Vec<Lecture>>(&s) {
            for lec in exist {
                lecture_map.insert(lec.code.to_string(), lec);
            }
        }
    }

    println!("{} before length: {}", filename, lecture_map.keys().len());

    let session = Arc::new(USaintSession::anonymous());
    // 일단 0부터 9까지 검색한거 모으고 code를 기준으로 중복 제거
    let lectures2: Vec<Vec<Lecture>> = (0..=3)
        .map(|i| find_by_lecture(session.clone(), year, semester, i.to_string(), i))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    for lectures in lectures2 {
        for lec in lectures {
            if lecture_map.contains_key(&lec.code) {
                continue;
            }
            lecture_map.insert(lec.code.to_string(), lec);
        }
    }

    println!("total length: {}", lecture_map.keys().len());

    let mut lectures = lecture_map.values().collect::<Vec<&Lecture>>();
    lectures.sort_by(|x, y| x.code.cmp(&y.code));
    let json =
        serde_json::to_string_pretty(&lectures).expect("Failed to serialize lectures to JSON");
    let mut file = File::create(&filename).expect("Failed to create json file");
    file.write_all(json.as_bytes())
        .expect("Failed to write file");

    Ok(lecture_map)
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

/*
// 타전공인정 과목
async fn collect_recognized_other_major(year: u32, semester: SemesterType) -> Result<(), Box<RusaintError>> {
    println!("collect recognized other major: {} {}", year, semester);
    let session = Arc::new(USaintSession::anonymous());
    let mut app = USaintClientBuilder::new()
        .session(session.clone())
        .build_into::<CourseScheduleApplication>()
        .await?;

    let mut other_major: HashMap<String, Vec<Major>> = HashMap::new();
    for college in app.collages(year, semester).await? {
        for department in app.departments(year, semester, &college).await? {
            for major in app.majors(year, semester, &college, &department).await? {
                let lectures = find_recognized_other_major(
                    session.clone(),
                    year,
                    semester,
                    &college,
                    &department,
                    &major,
                )
                .await?;
                for lec in lectures {
                    let major = Major {
                        college: college.to_string(),
                        department: department.to_string(),
                        major: major.to_string(),
                    };
                    match other_major.get_mut(lec.code()) {
                        Some(ref mut v) => v.push(major),
                        None => {
                            other_major.insert(lec.code().to_string(), vec![major]);
                        }
                    }
                }
            }
        }
    }

    let json =
        serde_json::to_string_pretty(&other_major).expect("Failed to serialize lectures to JSON");
    std::fs::create_dir_all("./assets/other_major").expect("Failed to create dir");
    let mut file = File::create(format!(
        "./assets/other_major/{}_{}.json",
        year,
        semester_to_code(semester)
    ))
    .expect("Failed to create json file");
    file.write_all(json.as_bytes())
        .expect("Failed to write file");

    Ok(())
}

async fn find_recognized_other_major(
    session: Arc<USaintSession>,
    year: u32,
    semester: SemesterType,
    college: &str,
    department: &str,
    major: &str,
) -> Result<Vec<Lecture>, RusaintError> {
    println!("({}, {}, {})", college, department, major);
    let mut app = USaintClientBuilder::new()
        .session(session)
        .build_into::<CourseScheduleApplication>()
        .await?;
    let category = LectureCategory::recognized_other_major(college, department, Some(major));
    let lectures = match app.find_lectures(year, semester, &category).await {
        Ok(lectures) => lectures.collect(),
        Err(_) => vec![],
    };
    Ok(lectures)
}
*/

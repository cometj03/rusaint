use rusaint::model::SemesterType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Major {
    pub college: String,
    pub department: String,
    pub major: String,
}

enum GradeScale {
    Score,
    Pf,
    Unknown,
}
enum GradeRule {
    Relative,
    Absolute,
    Unknown,
}
enum Lang {
    Korean,
    English,
    MixedEnglishKorean,
    Nation,
    MixedNationKorean,
    Unknown,
}
enum Process {
    Bachelor,
    Master,
    SukBak,
    Unknown,
}
struct ScheduleRoom {
    day: String,          // 요일: 월화수목금토일
    room: Option<String>, // 강의실
    start_time: String,
    end_time: String,
}
struct CourseCategory {
    is_main: bool, // true: 주전공, false: 다전공
}

struct Course {
    year: u32,
    semester: SemesterType,
    grade_scale: GradeScale, // 성적 스케일 "SCALE_TEXT"
    grade_rule: GradeRule,   // 성적평가방식 "CLSBORDT"
    lang: Lang,              // 강의언어 "CLSLANGT"
    process: Process,        // 과정
    is_el: bool,
    is_capstone: bool,
    limited_target: bool, // 대상 외 수강제한
    syllabus: Syllabus,
    code: String,
    name: String,
    division: Option<String>, // 분반
    professor: String,
    department: String,               // 개설학과
    credit: f32,                      // 학점 (시간/학점에서 뒤에있는거)
    personnel: u32,                   // 수강인원
    remaining_seats: u32,             // 여석
    schedule_room: Vec<ScheduleRoom>, // 강의시간(강의실)
    target: String,                   // 수강대상
    note: Option<String>,             // 수강 유의사항
    type_information: Option<String>, // 강좌유형정보
    abeek_info: Option<String>,       // 공학인증
    field: Option<String>,            // 교과영역
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Syllabus {
    pub name: String,
    pub professor: String,
    pub year: String,
    pub semester: String,
    pub code: String,
    pub division_no: String,            // 분반: '01', '02'
    pub target: String,                 // 수강대상학과: '3학년 컴퓨터', '컴퓨터', '3학년' 
    pub category: String,               // 이수구분: '전필-컴퓨터'
    pub grade_scale: String,            // 성적스케일: '점수 100기준 입력'
    pub lang: String,                   // 강의언어
    pub prof_room: String,              // 교수실
    pub prof_telno: String,             // 연락처
    pub prof_email: String,             // 이메일
    pub credit: String,                 // 학점/주당시간
    pub subject_type: String,           // 교과목유형: '이론'
    pub grade_rule: String,             // 성적평가방식
    pub counsel: String,                // 상담 신청 방식
    pub required_prerequisite: String,  // 필수 선수과목
    pub recommend_prerequisite: String, // 권장 선수과목
    pub class_abstract: String,         // 교과목 개요
    pub class_way_type: String,         // 강좌형식: '이론, 토론식수업'
    pub textbook_main: String,          // 주교재
    pub textbook_sub: String,           // 참고교재(대표)
    pub preclass: String,               // 학습준비사항
    pub etc: String,                    // 수강생 유의 및 참고사항
    pub class_type: String,             // 수업유형
    pub grading: Vec<Grading>,
    pub week: Vec<Week>,
    pub goal: Vec<Goal>,
    pub file: Vec<SyllabusFile>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Grading {
    name: String,       // 평가항목: '중간고사', '출석'
    max_score: String,  // 각 항목별 만점(최대 100점)
    rate: String,       // 반영비율(합계100%)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Week {
    week_no: String,        // 주: '01'
    keyword: String,        // 핵심어
    description: String,    // 세부내용
    teaching_way: String,   // 교수방법: '강의, 토론, 시험'
    textbook: String,       // 교재범위
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Goal {
    goal: String,   // 교육목표
    skills: String, // 전공특화역량
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SyllabusFile {
    name: String,
    url: String,
}

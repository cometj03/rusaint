use rusaint::model::SemesterType;

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
                                      // multi_major: Vec<>
}

struct Syllabus {
    name: String,
    professor: String,
    grade_scale: String,
    division_no: String,
    target: String,
    category: String,
    grade_rule: String,
    lang: String,
    prof_room: String,
    prof_telno: String,
    prof_email: String,
    required_prerequisite: String,
    recommend_prerequisite: String,
    abstract: String,
    subject_type: String,
    class_progress_type: String,
    class_type: String,
    textbook_main: String,
    textbook_sub: String,
    prepare_for_study: String,
    note: String,
}

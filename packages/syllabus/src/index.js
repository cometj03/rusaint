const fs = require("fs");
const qs = require("querystring");
const getSyllabus = require("../js/ozviewer/getSyllabus");

const parseCode = (url) => {
    let ur = new URL(url);
    let params = qs.parse(ur.search.slice(1));
    let v = params.pValue;
    return v.split(",")[2];
};

const processSyllabus = (syllabus) => {
    let ret = {};
    for (let [k, v] of [
        ["SMTEXT", "name"],
        ["PROF_NM", "professor"],
        ["PERYR", "year"],
        ["PERID", "semester"],
        ["SMOBJID", "code"],
        ["SENO", "division_no"],                // 분반: '01', '02'
        ["BOOK_TARGET", "target"],              // 수강대상학과: '3학년 컴퓨터', '(학과)', '3학년' 
        ["DESIGNATION", "category"],            // 이수구분: '전필-컴퓨터'
        ["SCALE_TEXT", "grade_scale"],          // 성적스케일: '점수 100기준 입력'
        ["CLSLANGT", "lang"],                   // 강의언어
        ["PROF_ROOM", "prof_room"],             // 교수실
        ["PROF_TELNR", "prof_telno"],           // 연락처
        ["SMTPADR", "prof_email"],              // 이메일
        ["PTPLAN", "credit"],                   // 학점/주당시간 
        ["CATEGORYT_1746", "subject_type"],     // 교과목유형: '이론'
        ["CLSBORDT", "grade_rule"],             // 성적평가방식
        ["COUNSELTM", "counsel"],               // 상담 신청 방식
        ["PREREQ", "required_prerequisite"],    // 필수 선수과목
        ["PREREQ_M", "recommended_prerequisite"], // 권장 선수과목
        ["ABSTRACT", "class_abstract"],         // 교과목 개요
        ["CLSWY_TEXT", "class_way_type"],       // 강좌형식: '이론, 토론식수업'
        ["TXTREFER", "textbook_main"],          // 주교재
        ["TXTREFER_M", "textbook_sub"],         // 참고교재(대표)
        ["PRECLASS", "preclass"],               // 학습준비사항
        ["CHAMGO", "etc"],                      // 수강생 유의 및 참고사항
        ["CLASSTYPE", "class_type"],            // 수업유형
    ]) {
        ret[v] = syllabus.ET_PLAN[0][0][k];
    }
    
    ret.grading = [];
    for (let d of syllabus.ET_APP[0]) {
        let tmp = {};
        for (let [k, v] of [
            ["AGRDESC", "name"], // 평가항목: '중간고사', '출석'
            ["RADD", "max_score"], // 각 항목별 만점(최대 100점)
            ["RATE", "rate"], // 반영비율(합계 100%)
        ]) {
            tmp[v] = d[k];
        }
        ret.grading.push(tmp);
    }

    ret.week = [];
    for (let d of syllabus.ET_WEEK[0]) {
        let tmp = {};
        for (let [k, v] of [
            ["WEEKLY", "week_no"], // 주: '01'
            ["COREWORD", "keyword"], // 핵심어
            ["DETAILS", "description"], // 세부내용
            ["REMARKT", "teaching_way"], // 교수방법: '강의, 토론, 시험'
            ["TEXTAREA", "textbook"], // 교재범위
        ]) {
            tmp[v] = d[k];
        }
        ret.week.push(tmp);
    }

    ret.goal = [];
    for (let d of syllabus.ET_GOAL[0]) {
        let tmp = {};
        for (let [k, v] of [
            ["GOAL", "goal"], // 교육목표
            ["JUNTXT", "skills"], // 전공특화역량
        ]) {
            tmp[v] = d[k];
        }
        ret.goal.push(tmp);
    }

    ret.file = [];
    for (let d of syllabus.ET_FILE[0]) {
        let tmp = {};
        for (let [k, v] of [
            ["FILE_NAME", "name"],
            ["FILE_URL", "url"],
        ]) {
            tmp[v] = d[k];
        }
        ret.file.push(tmp);
    }

    return ret;
};

(async () => {
    fs.mkdirSync(`${__dirname}/../assets/syllabus`, {recursive: true});

    for (let filename of fs.readdirSync(`${__dirname}/../assets/lectures`)) {
        console.log(`filename: ${filename}`);
        let [year, semester] = filename.split(".")[0].split("_");
        let data = JSON.parse(fs.readFileSync(`${__dirname}/../assets/lectures/${filename}`, "utf-8"));

        let syllabus_data = {};

        const run = async (r) => {
            if (!data.length) {
                r();
                return;
            }
            let {syllabus, code} = data.pop();
            if (!syllabus) {
                run(r);
                return;
            }
            let syllabus_code = parseCode(syllabus);
            let raw_data = await getSyllabus(year, semester, syllabus_code);
            if (raw_data) syllabus_data[code] = processSyllabus(raw_data);
            run(r);
        }

        let promises = [];
        for (let i = 0; i < 10; i++)
            promises.push(new Promise(r => run(r)));
        await Promise.all(promises);

        fs.writeFileSync(`${__dirname}/../assets/syllabus/${filename}`, JSON.stringify(syllabus_data, null, 2));
        console.log(`finish ${year} ${semester}`);
    }
})();
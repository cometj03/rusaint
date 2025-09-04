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
        ["BOOK_TARGET", "target"],
        ["DESIGNATION", "isu_main_type"],
        ["PTPLAN", "credit"],
        ["SCALE_TEXT", "grade_scale"],
        ["CLSLANGT", "lang"],
        ["PROF_ROOM", "prof_room"],
        ["PROF_TELNR", "prof_telno"],
        ["SMTPADR", "prof_email"],
        ["CATEGORYT_1746", "subject_type"],
        ["CLSBORDT", "grade_rule"],
        ["PREREQ", "must_listen_before"],
        ["PREREQ_M", "should_listen_before"],
        ["ABSTRACT", "abstract"],
        ["CLSWY_TEXT", "class_progress_type"],
        ["TXTREFER", "textbook_main"],
        ["TXTREFER_M", "textbook_sub"],
        ["PRECLASS", "need_for_study"],
        ["CHAMGO", "etc"],
        ["CLASSTYPE", "class_type"],
    ]) {
        let val = syllabus.ET_PLAN[0][0][k].trim();
        if (val.length > 0) {
            ret[v] = val;
        }
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
            let a = await getSyllabus(year, semester, syllabus_code);
            if (a) syllabus_data[code] = processSyllabus(a);
            run(r);
        }

        let promises = [];
        for (let i = 0; i < 10; i++)
            promises.push(new Promise(r => run(r)));
        await Promise.all(promises);

        fs.writeFileSync(`${__dirname}/../assets/syllabus/${filename}`, JSON.stringify(syllabus_data, null, 4));
        console.log(`finish ${year} ${semester}`);
    }
})();
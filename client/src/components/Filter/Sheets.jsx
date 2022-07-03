import { utils, writeFileXLSX } from "xlsx";

function Sheets(props) {
    const sheetsHandler = () => {
        const data = [];
        console.log(props.data);
        for(let entry of props.data){
            let new_entry = {};
            for(let key of Object.keys(entry)){
                if(props.field().includes(key)){
                    new_entry[key] = entry[key];
                }
            }
            console.log(new_entry)
            data.push(new_entry);
        }
        let workBook = utils.book_new();
        let worksheet = utils.json_to_sheet(data);

        
        utils.book_append_sheet(workBook, worksheet, "sheet1");
        writeFileXLSX(workBook, "Placement.xlsx");
    }
    return (
        <button class="rounded-circle bg-success d-flex justify-content-center align-items-center p-3" onClick={sheetsHandler} style={{position: "fixed", bottom: "30px", right: "30px"}}>
            <svg width="16" height="16" fill="currentColor" class="bi bi-download" viewBox="0 0 16 16">
                <path d="M.5 9.9a.5.5 0 0 1 .5.5v2.5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2.5a.5.5 0 0 1 1 0v2.5a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-2.5a.5.5 0 0 1 .5-.5z"/>
                <path d="M7.646 11.854a.5.5 0 0 0 .708 0l3-3a.5.5 0 0 0-.708-.708L8.5 10.293V1.5a.5.5 0 0 0-1 0v8.793L5.354 8.146a.5.5 0 1 0-.708.708l3 3z"/>
            </svg>
        </button>
    );
}

export default Sheets;
import { Show, For, createSignal, createEffect, lazy, Suspense } from "solid-js";
import { Portal } from "solid-js/web";
import { useNavigate } from "solid-app-router";

import { Capitalize, Store } from "../../utils";
import { ColumnList } from "../../Helpers";
// import Sheets from "./Sheets";

const Sheets = lazy(() => import("./Sheets"));

function Table() {
    const navigate = useNavigate();
    const { getStudents, getIsAuthed, setDisplay } = Store;
    setDisplay(false);
    if(!getIsAuthed()){
        navigate("/");
    }
    const [getIsEmpty, setIsEmpty] = createSignal(true);
    const [getFields, setFields] = createSignal(["id"]);
    const [getSelectedColumn, setSelectedColumn] = createSignal([getFields()[0]]);
    createEffect(() => {
        if(getStudents().length > 0){
            setIsEmpty(false);
            setSelectedColumn([getFields()[0]])
        }else{
            setIsEmpty(true);
        }
    })

    createEffect(() => {
        if(!getIsEmpty()){
            setFields(Object.keys(getStudents()[0]));
        }
    })
    const backBtnHandler=() => {
        navigate("/filter");
    }
    return (
    <div style={{margin: "60px 0px"}}>
        <Portal>
            <button 
                onclick={backBtnHandler}
                style={{position: "fixed", left: "30px", bottom: "30px"}} 
                class="bg-success rounded-circle d-flex justify-content-center align-items-center p-3"
            ><svg width="16" height="16" fill="currentColor" class="bi bi-arrow-left" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M15 8a.5.5 0 0 0-.5-.5H2.707l3.147-3.146a.5.5 0 1 0-.708-.708l-4 4a.5.5 0 0 0 0 .708l4 4a.5.5 0 0 0 .708-.708L2.707 8.5H14.5A.5.5 0 0 0 15 8z"/>
            </svg>
            </button>
        </Portal>
        <Show when={!getIsEmpty()} fallback={<>There are no records</>}>
        <ColumnList list={getFields} model={[getSelectedColumn, setSelectedColumn]} />
            <Suspense><Portal><Sheets data={getStudents()} field={getSelectedColumn} /></Portal></Suspense>
            <table class="table">
                <thead>
                    <For each={getSelectedColumn()}>
                        {(field) => <th scope="col" class="table-success">{Capitalize(field)}</th>}
                    </For>
                </thead>
                <tbody class="table-group-divider">
                    <For each={getStudents()}>
                        {(student) => {
                            return (
                                <tr class="table-success">
                                    <For each={getSelectedColumn()}>
                                        {(value) => <td class="table-success">{student[value]}</td>}
                                    </For>
                                </tr>
                            )
                        }}
                    </For>
                </tbody>
            </table>
        </Show>
    </div>
    );
}

export default Table;
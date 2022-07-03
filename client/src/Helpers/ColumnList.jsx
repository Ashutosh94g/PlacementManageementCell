import { For } from "solid-js";
import { Capitalize } from "../utils";

function ColumnList(props) {

    const checkHandler = (event) => {
        if( event.target.checked ) {
            if(!props.model[0]().includes(event.target.value)){
                let newList = [...props.model[0](), event.target.value];
                props.model[1](newList);
            }
        }else {
            props.model[1]((list) => list.filter(item => item !== event.target.value))
        }
        // console.log(props.model[0]());
    }

    return (
        <div>
            <label class="form-label">Please Select the Columns:</label>
            <br />
            <For each={props.list()}>
                {(item, index) => {
                return (
                    <div class="form-check form-check-inline m-3">
                        <input class="form-check-input" type="checkbox" value={item} id="flexCheckDefault" onchange={checkHandler} checked={index() == 0} />
                        <label class="form-check-label" for="flexCheckDefault">
                            {Capitalize(item)}
                        </label>
                    </div>
                )
                }}
            </For>
        </div>
    )
}

export default ColumnList;
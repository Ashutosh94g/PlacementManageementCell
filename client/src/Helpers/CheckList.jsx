import { For } from "solid-js";

function CheckList(props) {
    
    const checkHandler = (event) => {
        if( event.target.checked ) {
            if(!props.model[0]().includes(event.target.value)){
                let newList = [...props.model[0](), parseInt(event.target.value)];
                props.model[1](newList);
            }
        }else {
            props.model[1]((list) => list.filter(item => item !== parseInt(event.target.value)))
        }
        console.log(props.model[0]());
    }

    return (
        <div>
            <label class="form-label">{props.name}</label>
            <For each={props.list()}>
                {item => {
                return (
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" value={item.id} id="flexCheckDefault" onchange={checkHandler} />
                        <label class="form-check-label" for="flexCheckDefault">
                            {item.value}
                        </label>
                    </div>
                )
                }}
            </For>
        </div>
    )
}

export default CheckList;
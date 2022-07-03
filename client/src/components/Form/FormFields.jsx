import { createEffect, createSignal, For } from "solid-js";
import { Model } from "../../Helpers";

function Field(props) {
    let def;
    if(props.type === "number") {
        def = 0;
    }else{
        def = "";
    }
    const [getter, setter] = createSignal(def);

    props.error

    createEffect(() => {
        // putting value into data object
        if(props.type==="number"){
            props.data[props.keyname] = parseInt(getter());
        }else{
            props.data[props.keyname] = getter();
        }
    })
    let req;
    if(props.required === undefined || props.required === true){
        req = true;
    }else if(props.required === false){
        req = false;
    }
    return (
        <div class={props.class ? props.class : "col-md-6"}>
			<label class="form-label" for={props.name}>{props.name}</label>
			<input 
                class="form-control" 
                type={props.type} 
                id={props.name} 
                use:Model={[getter, setter]} 
                placeholder={props.name} 
                required={req}
            />
        </div>
    );
}

function Select(props) {
    const [getter, setter] = createSignal(0);
    createEffect(() => {
        props.data[props.keyname] = parseInt(getter());
    }) 
    return (
		<div class={props.class ? props.class : "col-md-6"}>
			<label class="form-label" for={props.name}>{props.name}</label>
			<select class="form-select" id={props.name} use:Model={[getter, setter]}>
				<option value="0">Please select a {props.name}</option>
				<For each={props.list()}>
					{(item) => <option value={item.id}>{item.value}</option>}
				</For>
			</select>
		</div>
    );
}

function NField(props) {
    const [getter, setter] = createSignal(props.min);
    createEffect(() => {
        props.data[props.keyname] = parseFloat(getter());
    })
    return (
        <div class={props.class ? props.class : "col-md-6"}>
        <label class="form-label" for={props.name}>{props.name}</label>
        <input
            class="form-control"
            type="number"
            id={props.name}
            use:Model={[getter, setter]}
            step={props.step}
            min={props.min}
            max={props.max}
        />
        </div>
    );
}

function Checkbox(props) {
    const [getter, setter] = createSignal(true);

    createEffect(() => {
        props.data[props.keyname] = getter() ? 1 : 0;
    })
    return (
        <div class="form-check m-3">
            <input class="form-check-input" type="checkbox" onChange={() => setter(v => !v)} id="flexCheckDefault" checked />
            <label class="form-check-label" for="flexCheckDefault">
                {props.name}
            </label>
        </div>
    );
}

function CheckList(props) {
    const [getter, setter] = createSignal([]);
    createEffect(() => {
        props.data[props.keyname] = getter();
    })
    const checkHandler = (event) => {
        if( event.target.checked ) {
            if(!getter().includes(event.target.value)){
                let newList = [...getter(), parseInt(event.target.value)];
                setter(newList);
            }
        }else {
            setter((list) => list.filter(item => item !== parseInt(event.target.value)))
        }
        console.log(getter());
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
    );
}

export { Field, Select, NField, Checkbox, CheckList };
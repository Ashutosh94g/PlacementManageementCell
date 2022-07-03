import { For } from "solid-js";

import Model from "./Model";

function Select(props) {
    return (
		<div class={props.class ? props.class : "col-md-6"}>
			<label class="form-label" for={props.name}>{props.name}</label>
			<select class="form-select" id={props.name} use:Model={props.model}>
				<option value="0">Please select a {props.name}</option>
				<For each={props.list()}>
					{(item) => <option value={item.id}>{item.value}</option>}
				</For>
			</select>
		</div>
    );
}

export default Select;
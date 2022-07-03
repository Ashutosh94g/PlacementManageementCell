import Model from "./Model";

function Field(props) {
  	return (
		<div class={props.class ? props.class : "col-md-6"}>
			<label class="form-label" for={props.name}>{props.name}</label>
			<input class="form-control" type={props.type} id={props.name} use:Model={props.model} placeholder={props.name} />
		</div>
	);
}

export default Field;
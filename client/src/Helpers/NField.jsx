import Model from "./Model";

function NField(props) {
    return (
        <div class={props.class ? props.class : "col-md-6"}>
        <label class="form-label" for={props.name}>{props.name}</label>
        <input
            class="form-control"
            type="number"
            id={props.name}
            use:Model={props.model}
            step={props.step}
            min={props.min}
            max={props.max}
        />
        </div>
    );
}

export default NField;
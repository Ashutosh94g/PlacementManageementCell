import Model from "./Model";

function LargeField(props) {
    return (
        <div>
            <label for={props.name}>{props.name}</label>
            <textarea id={props.name} use:Model={props.model} row="2"></textarea>
        </div>
    );
}

export default LargeField;
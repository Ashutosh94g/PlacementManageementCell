import clickOutside from "./Clickoutside";

function InfoModal(props) {
    return (
        <>
        <div class="modal" style={{zIndex: 2, display: "block"}}>
            <div class="modal-dialog">
                <div class="modal-content" use:clickOutside={props.confirm}>
                    <div class="modal-header">
                        <h5 class="modal-title">{props.title}</h5>
                        <button type="button" class="btn-close" onclick={props.confirm}></button>
                    </div>
                    <div class="modal-body">
                        <p>{props.body}</p>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class={`btn btn-${props.color}`} onclick={props.confirm}>Close</button>
                    </div>
                </div>
            </div>
        </div>
        </>
    );
}

export default InfoModal;
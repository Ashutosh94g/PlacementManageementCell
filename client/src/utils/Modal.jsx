import { onCleanup } from "solid-js";

export function clickOutside(el, accessor) {
    const onClick = (e) => !el.contains(e.target) && accessor();
    document.body.addEventListener("click", onClick);

    onCleanup(() => document.body.removeEventListener("click", onClick));
  }

function Modal(props) {
    const closeModel = () => {
        props.setter(false);
    }
    const confirmModel = () => {
        props.confirm();
    }
    return (
        <div class="modal" style={{zIndex: 2, display: "block"}}>
            <div class="modal-dialog">
                <div class="modal-content" use:clickOutside={props.setter(false)}>
                    <div class="modal-header">
                        <h5 class="modal-title">{props.title}</h5>
                        <button type="button" class="btn-close" onclick={closeModel}></button>
                    </div>
                    <div class="modal-body">
                        <p>{props.body}</p>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" onclick={closeModel}>Cancel</button>
                        <button type="button" class="btn btn-success" onclick={confirmModel}>Confirm</button>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Modal;
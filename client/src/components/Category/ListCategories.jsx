import { For } from "solid-js";

import { Store } from "../../utils";

function ListCategories(props) {
    const {getToken} = Store;
    const deleteItem = (event) => {
        const filteringId = event.currentTarget.value;
        const filteredList = props.list[0]().filter(item => item.id != filteringId);
        props.list[1](filteredList);
        fetch(`http://localhost:8080/api/${props.table}/${filteringId}`, {
            method: "DELETE",
            headers: {
                "Authorization": `bearer ${getToken()}`
            }
        }).then(response => {
            return {status: response.status, body: response.json()};
        }).then(result => {
            if(result.status === 200){
                alert("The item is deleted");
            }else {
                throw result;
            }
        }).catch(error => {
            alert("There was an error while deleting");
            console.log(error);
        })
    }
    return (
        <ul class="list-group">
            <For each={props.list[0]()}>
                {(item) => {
                    return (
                        <>
                            <li class="list-group-item d-flex justify-content-between align-items-center">
                                <h6 class="align-self-center">{item.value}</h6>
                                <button class="btn btn-danger" value={item.id} onclick={deleteItem}>
                                    <svg width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                        <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                        <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                                    </svg>
                                </button>
                            </li>
                        </>
                    );
                }}
            </For>
        </ul>
    )
}

export default ListCategories;
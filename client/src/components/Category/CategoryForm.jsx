import {createSignal} from "solid-js";

import { Store } from "../../utils";
import { Model } from "../../Helpers";

function contains(accesss, value) {
    for(let acc of accesss()){
        if(acc.value == value){
            return true;
        }
    }
    return false;
}

function CategoryForm(props) {
    const {getToken} = Store;
    

    const value = createSignal("");

    const submitHandler = (event) => {
        event.preventDefault();
        if(value[0]() == ""){
            alert("Please Enter a value");
            value[1]("");
            return;
        }
        
        if(contains(props.list[0], value[0]())){
            alert("The element already exists");
            value[1]("");
            return;
        }

        let data = {
            value: value[0]()
        }
        fetch(`http://localhost:8080/api/${props.table.toLowerCase()}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `bearer ${getToken()}`
            },
            body: JSON.stringify(data)
        }).then(response => {
            return {status: response.status, body: response.json()};
        }).then(result => {
            if(result.status == 201){
                props.list[1](l => [...l, data]);
            }else{
                throw result;
            }
        }).catch(error => {
            console.log(error);
            alert("An error occured")
        })
        value[1]("");
    }

    return (
        <form id="category-form" class="card p-4" onsubmit={submitHandler}>
            <div class="form-floating">
                <input class="form-control" type="text" use:Model={value} placeholder={props.table} />
                <label>{props.table}</label>
            </div>
            <button class="btn btn-success mt-4">Add {props.table}</button>
        </form>
    )
}

export default CategoryForm;
import {createSignal, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { useNavigate } from "solid-app-router";


import { Store, InfoModal } from "../../utils";
import { Model } from "../../Helpers";
import Verify from "./Verify";

function Login() {
    // Local State 
    const [getEmail, setEmail] = createSignal("");
    const [getPassword, setPassword] = createSignal("");
    const [getTrueModal, setTrueModal] = createSignal(false);
    const [getFalseModal, setFalseModal] = createSignal(false);

    // extractors
    const navigate = useNavigate();
    const {setAuthToken, setDisplay} = Store;
    
    setDisplay(false);
    // Handlers
    const confirmHandler = () => {
        setTrueModal(false);
        setFalseModal(false);
        setDisplay(false);
        navigate("/");
    }

    const submitHandler = (event) => {
        event.preventDefault();
        if(!Verify(getEmail(), getPassword())) return;

        let data = {
            "email": getEmail(),
            "password": getPassword()
        }

        fetch("http://localhost:8080/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        }).then(response => {
            console.log(response);
            if(response.status === 200){
                return response.json();
            }
            throw response.json();
        }).then(result => {
            setAuthToken(result);
            setTrueModal(true);
        }).catch(error => {
            setFalseModal(true);
            console.dir(error);
        })
    }

    return (
    <main>
        <form class="card col-md-3" id="login" onsubmit={submitHandler}>
            <h3 class="h3 mb-3 fw-normal card-title">Login</h3>
            <label class="form-label">Email</label>
            <input type="text" class="form-control"  use:Model={[getEmail, setEmail]} placeholder="Email" />
            <label class="form-label mt-3">Password</label>
            <input type="password" class="form-control" use:Model={[getPassword, setPassword]} placeholder="Password" />
            <button class="w-100 btn btn-lg btn-success" style={{"margin-top": "20px"}} type="submit">Login</button>
        </form>
        <Show when={getTrueModal()} fallback={<></>}>
            <Portal>
                <InfoModal title="Login Successful" body="You are successfully Logged In" color="success" confirm={confirmHandler} />
            </Portal>
        </Show>
        <Show when={getFalseModal()} fallback={<></>}>
            <Portal>
                <InfoModal title="Login Failed" body="Your login was unsuccessful! Please check the credentials" color="danger" confirm={confirmHandler} />
            </Portal>
        </Show>
    </main>
    )
}

export default Login;
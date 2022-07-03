import { createSignal, Show } from "solid-js"
import { useNavigate } from "solid-app-router";

import { Store, InfoModal } from "../../utils";
import { Model } from "../../Helpers";
import Verify from "./Verify";

function Register() {
    const {setAuthToken, setDisplay} = Store;
    const navigate = useNavigate();

    const [getEmail, setEmail] = createSignal("");
    const [getPassword, setPassword] = createSignal("");
    const [getConfirmPassword, setConfirmPassword] = createSignal("");
    const [getTrueModal, setTrueModal] = createSignal(false);
    const [getFalseModal, setFalseModal] = createSignal(false);

    setDisplay(false);

    const confirmHandler = () => {
        setTrueModal(false);
        setFalseModal(false);
        setDisplay(false);
        navigate("/");
    }

    const submitHandler = (event) => {
        event.preventDefault();
        const confirm = getPassword() === getConfirmPassword();

        if(!confirm) {
            alert("Passwords don't match");
            return;
        }

        if(!Verify(getEmail(), getPassword())) return;

        let data = {
            email: getEmail(),
            password: getPassword()
        }

        fetch("http://localhost:8080/api/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        }).then(response => {
            return {status: response.status, body: response.json()};
        }).then(result => {
            if(result.status === 409){
                throw result;
            }else if(result.status === 201){
                setAuthToken(result.body);
                setTrueModal(true);
            }else{
                alert(result.body);
            }
        }).catch(error => {
            setFalseModal(true);
            console.log(error);
        })
    }

    return (
    <main>
        <form id="register" class="card col-md-3" onsubmit={submitHandler}>
            <label class="form-label">Email</label>
            <input type="text" class="form-control" use:Model={[getEmail, setEmail]} />
            <label class="form-label mt-3">Password</label>
            <input type="password" class="form-control" use:Model={[getPassword, setPassword]} />
            <label class="form-label mt-3">Confirm Password</label>
            <input type="password" class="form-control" use:Model={[getConfirmPassword, setConfirmPassword]} />
            <button class="w-100 btn btn-lg btn-success mt-4" type="submit">SignUp</button>
        </form>
        <Show when={getTrueModal()} fallback={<></>}>
            <Portal>
                <InfoModal title="Registration Successful" body="You are successfully Registered" color="success" confirm={confirmHandler} />
            </Portal>
        </Show>
        <Show when={getFalseModal()} fallback={<></>}>
            <Portal>
                <InfoModal title="Registration Failed" body="Your account is already created! Please try to login" color="danger" confirm={confirmHandler} />
            </Portal>
        </Show>
    </main>
    )
}

export default Register;
import { createSignal, createEffect, createRoot } from "solid-js";

function CreateStore() {
    const [getToken, setToken ] = createSignal("");
    const [getIsAuthed, setIsAuthed] = createSignal(false);
    const [getGenders, setGenders] = createSignal([]);
    const [getCategories, setCategories] = createSignal([]);
    const [getFatherOccupations, setFatherOccupations] = createSignal([]);
    const [getMotherOccupations, setMotherOccupations] = createSignal([]);
    const [getBoards, setBoards] = createSignal([]);
    const [getQualifications, setQualifications] = createSignal([]);
    const [getSpecializations, setSpecializations] = createSignal([]);
    const [getDisplay, setDisplay] = createSignal(false);
    const [getStudents, setStudents] = createSignal([]);
    const [getCampuses, setCampuses] = createSignal([]);

    const setAuthToken = (value) => {
        if (value.split(".").length !== 3){
            alert("The user is not Verified");
            return;
        }
        setToken(value);
    }

    createEffect(() => {
        if(getToken().split(".").length === 3) {
            setIsAuthed(true);
        }else {
            setIsAuthed(false);
        }
    })

    const logout = () => {
        setToken("");
        setIsAuthed(false);
    }

    createEffect(() => {
        fetch("http://localhost:8080/api/gender")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setGenders(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });
    createEffect(() => {
        fetch("http://localhost:8080/api/category")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setCategories(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });
    createEffect(() => {
        fetch("http://localhost:8080/api/father_occupation")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setFatherOccupations(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });
    createEffect(() => {
        fetch("http://localhost:8080/api/mother_occupation")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setMotherOccupations(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });
    createEffect(() => {
        fetch("http://localhost:8080/api/board")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setBoards(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });
    createEffect(() => {
        fetch("http://localhost:8080/api/qualification")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setQualifications(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });
    createEffect(() => {
        fetch("http://localhost:8080/api/specialization")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setSpecializations(result);
        })
        .catch((error) => {
            console.log(error);
        });
    });

    createEffect(() => {
        fetch("http://localhost:8080/api/campus")
        .then((data) => {
            return data.json();
        })
        .then((result) => {
            setCampuses(result);
        })
        .catch(error => {
            console.log(error);
        })
    })

    return {getGenders, setGenders,
        getBoards, setBoards,
        getCategories, setCategories,
        getFatherOccupations, setFatherOccupations,
        getMotherOccupations, setMotherOccupations,
        getQualifications, setQualifications,
        getSpecializations, setSpecializations,
        setAuthToken, getToken, 
        getIsAuthed, setIsAuthed, 
        getDisplay, setDisplay,
        getStudents, setStudents,
        getCampuses, setCampuses,
        logout}
}

export default createRoot(CreateStore);
import { createSignal, For } from "solid-js";
import { useNavigate } from "solid-app-router";


import CategoryForm from "./CategoryForm";
import ListCategories from "./ListCategories";
import Store from "../../utils/Store";
import { Model } from "../../Helpers";


function AddCategory() {
    const navigate = useNavigate();
    const {
        getBoards, setBoards,
        getCategories, setCategories,
        getQualifications, setQualifications,
        getSpecializations, setSpecializations,
        getGenders, setGenders,
        getFatherOccupations, setFatherOccupations,
        getMotherOccupations, setMotherOccupations,
        getIsAuthed, setDisplay,
        getCampuses, setCampuses
    } = Store;
    
    if(!getIsAuthed()){
        navigate("/");
    }

    setDisplay(false);
    const categories = {
        "Board": [getBoards, setBoards], 
        "Category": [getCategories, setCategories], 
        "Qualification": [getQualifications, setQualifications], 
        "Specialization": [getSpecializations, setSpecializations], 
        "Gender": [getGenders, setGenders], 
        "Father_occupation": [getFatherOccupations, setFatherOccupations], 
        "Mother_occupation": [getMotherOccupations, setMotherOccupations],
        "campus": [getCampuses, setCampuses],
    };
    const [getCategory, setCategory] = createSignal("Board");
    return (
    <main id="category-main">
        <div class="card p-4 col-md-4">
        <select class="form-select" use:Model={[getCategory, setCategory]}>
            {/* <option value="">Please Select a category</option> */}
            <For each={Object.keys(categories)}>
                {category => <option value={category}>{category}</option>}
            </For>
        </select>
        <ListCategories table={getCategory().toLowerCase()} list={categories[getCategory()]} />
        </div>
        <CategoryForm table={getCategory()} list={categories[getCategory()]} />
    </main>
    )
}

export default AddCategory;
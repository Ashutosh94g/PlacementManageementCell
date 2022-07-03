import { createEffect, createSignal } from "solid-js";
import { useNavigate } from "solid-app-router";

import { Field, NField, Select, CheckList } from "../../Helpers";
import { Store } from "../../utils";

function Filter() {
  const navigate = useNavigate();
  const {getGenders,
    getBoards,
    getCategories,
    getFatherOccupations,
    getMotherOccupations,
    getQualifications,
    getSpecializations,
    getToken, getIsAuthed, setStudents, setDisplay} = Store;

    if(!getIsAuthed()){
      navigate("/");
    }
    const tenthYears = createSignal([]);
    createEffect(() => {
      if(!getIsAuthed()){
        return;
      }
      fetch("http://localhost:8080/api/tenth_year", {
        headers: {
          "Authorization": `bearer ${getToken()}`
        }
      }).then(response => {
        console.log(response.status == 200);
        if(response.status == 200){
          return response.json();
        }else{
          throw response.json();
        }
      }).then(result => {
        let years = [];
        for(let year of result ){
          years.push({id: year["tenth_year"], value: year["tenth_year"]});
        }
        tenthYears[1](years);
      }).catch(error => {
        console.log(error);
        alert("There was an error in loading tenth years");
      });
    })
    const twelfthYears = createSignal([]);
    createEffect(() => {
      if(!getIsAuthed()){
        return;
      }
      fetch("http://localhost:8080/api/twelfth_year", {
        headers: {
          "Authorization": `bearer ${getToken()}`
        }
      }).then(response => {
        if(response.status === 200){
          return response.json();
        }
        throw response.json();
      }).then(result => {
        let years = [];
        for(let year of result ){
          years.push({id: year["twelfth_year"], value: year["twelfth_year"]});
        }
        twelfthYears[1](years);
      }).catch(error => {
        console.log(error);
        alert("There was an error in loading twelfth years");
      });
    })
    setDisplay(false);
    const id = createSignal("");
    const firstname = createSignal("");
    const lastname = createSignal("");
    const email = createSignal("");
    const phone = createSignal("");
    
    const fatherName = createSignal("");
    const motherName = createSignal("");
    const city = createSignal("");
    const state = createSignal("");
    const zip = createSignal("");

    const tenthYear = createSignal([]);
    const tenthPercentage = createSignal(0);
    const twelfthYear = createSignal([]);
    const twelfthPercentage = createSignal(0);
    
    const ugStartYear = createSignal([]);
    const ugEndYear = createSignal([]);
    const ugCgpa = createSignal(0);

    
    const fatherOccupation = createSignal([]);
    const motherOccupation = createSignal([]);
    const gender = createSignal([]);
    const category = createSignal([]);
    const ugQualification = createSignal([]);
    const ugSpecialization = createSignal([]);
    const tenthBoard = createSignal([]);
    const twelfthBoard = createSignal([]);

    const submitHandler = async (event) => {
      event.preventDefault();
      const data = {
        id: id[0](),
        pg_id: null,
        firstname: firstname[0](),
        lastname: lastname[0](),
        email: email[0](),
        phone: phone[0](),
        father_name: fatherName[0](),
        mother_name: motherName[0](),
        city: city[0](),
        state: state[0](),
        zip: zip[0](),
        tenth_year: tenthYear[0](),
        tenth_percentage: tenthPercentage[0](),
        twelfth_year: twelfthYear[0](),
        twelfth_percentage: twelfthPercentage[0](),
        ug_startyear: ugStartYear[0](),
        ug_endyear: ugEndYear[0](),
        ug_cgpa: ugCgpa[0](),
        father_occupation_id: fatherOccupation[0](),
        mother_occupation_id: motherOccupation[0](),
        gender: gender[0](),
        category: category[0](),
        ug_qualification_id: ugQualification[0](),
        ug_specialization_id: ugSpecialization[0](),
        tenth_board_id: tenthBoard[0](),
        twelfth_board_id: twelfthBoard[0]()
      }
      console.log(data);
      const response = await fetch("http://localhost:8080/api/filter", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `bearer ${getToken()}`,
        },
        body: JSON.stringify(data)
      });
      const result = await response.json();
      if(response.status === 200){
        setStudents(result);
        navigate("/table");
      }else{
        alert("Please Check the form");
        console.log(result);
      }
    }
    
    return (
      <form onsubmit={submitHandler} class="row g-3" style={{margin: "60px 0px"}}>
      <Field class="col-md-3" name="University-Id" type="text" model={id} />
      <Field class="col-md-3" name="Graduation Start Year" type="month" model={ugStartYear} />
      <Field class="col-md-3" name="Graduation End Year" type="month" model={ugEndYear} />
      <Field
        class="col-md-3"
        name="Graduation Cgpa"
        step="0.01"
        min="0"
        max="100"
        model={ugCgpa}
      />
      <div class="d-flex justify-content-around">
        <CheckList
          name="Graduation Qualification"
          model={ugQualification}
          list={getQualifications}
        />
        <CheckList
          name="Graduation Specialization"
          model={ugSpecialization}
          list={getSpecializations}
        />
      </div>
      <Field class="col-md-3" name="Firstname" type="text" model={firstname} />
      <Field class="col-md-3" name="Lastname" type="text" model={lastname} />
      <Field class="col-md-3" name="Email" type="email" model={email} />
      <Field class="col-md-3" name="Phone" type="tel" model={phone} />

      <div class="d-flex justify-content-around">
        <CheckList name="Gender" model={gender} list={getGenders} />
        <CheckList name="Category" model={category} list={getCategories} />
      </div>
      <Field name="Father-Name" type="text" model={fatherName} />
      <Field name="Mother-Name" type="text" model={motherName} />
      <div class="d-flex justify-content-around">
        <CheckList
          name="Father-Occupation"
          model={fatherOccupation}
          list={getFatherOccupations}
        />
        <CheckList
          name="Mother-Occupation"
          model={motherOccupation}
          list={getMotherOccupations}
        />
      </div>
      <NField
        name="Tenth Percentage"
        step="0.01"
        min="0"
        max="100"
        model={tenthPercentage}
      />
      <NField
        name="Twelfth Percentage"
        step="0.01"
        min="0"
        max="100"
        model={twelfthPercentage}
        />
      <div class="d-flex justify-content-around">
        <CheckList name="Tenth-Board" model={tenthBoard} list={getBoards} />
        <CheckList name="Tenth Year" model={tenthYear} list={tenthYears[0]}/>
        <CheckList name="Twelfth-Board" model={twelfthBoard} list={getBoards} />
        <CheckList name="Twelfth Year" model={twelfthYear} list={twelfthYears[0]} />
      </div>

      <Field class="col-md-4" name="City" type="text" model={city} />
      <Field class="col-md-4" name="State" type="text" model={state} />
      <Field class="col-md-4" name="PinCode" type="text" model={zip} />
      <div class="col-12 d-flex justify-content-center mb-3">
        <button type="submit" class="btn btn-success">Search</button>
      </div>
  </form>
  );
}
export default Filter;
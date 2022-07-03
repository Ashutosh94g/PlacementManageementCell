import { createSignal, createEffect } from "solid-js";
import { useNavigate } from "solid-app-router";

import { Field, NField, CheckList, Checkbox } from "../Form/FormFields";
import { Store } from "../../utils";

function Filter() {
    let date = new Date().getFullYear();
    const data = {
        pg_id: null
    };

    const navigate = useNavigate();
    const {getGenders,
        getBoards,
        getCategories,
        getFatherOccupations,
        getMotherOccupations,
        getQualifications,
        getSpecializations,
        getCampuses,
        getToken, getIsAuthed, setStudents, setDisplay} = Store;

    if(!getIsAuthed()){
        navigate("/");
    }
    setDisplay(false);
    
    const tenthYears = createSignal([]);
    const twelfthYears = createSignal([]);
    const startyear = createSignal([]);
    const endyear = createSignal([]);
    const backlogs = createSignal([]);
    const gaps = createSignal([]);
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
    createEffect(() => {
        if(!getIsAuthed()){
            return;
          }
          fetch("http://localhost:8080/api/endyear", {
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
              years.push({id: year["ug_endyear"], value: year["ug_endyear"]});
            }
            endyear[1](years);
          }).catch(error => {
            console.log(error);
            alert("There was an error in loading Graduation End year");
          });
    })
    createEffect(() => {
        if(!getIsAuthed()){
            return;
        }
        fetch("http://localhost:8080/api/startyear", {
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
            for(let year of result) {
                years.push({id: year["ug_startyear"], value: year["ug_startyear"]})
            }
            startyear[1](years);
        }).catch(error => {
            console.log(error);
            alert("There was an error in loading Graduation startyear");
        })
    })
    createEffect(() => {
        if(!getIsAuthed()){
            return;
        }
        fetch("http://localhost:8080/api/backlogs", {
            headers: {
                "Authorization": `bearer ${getToken()}`
            }
        }).then(response => {
            if(response.status === 200){
                return response.json();
            }
            throw response.json();
        }).then(result => {
            let logs = [];
            for(let log of result) {
                logs.push({id: log["no_of_backlogs"], value: log["no_of_backlogs"]})
            }
            backlogs[1](logs);
        }).catch(error => {
            console.log(error);
            alert("There was an error in loading No. of backlogs");
        })
    })
    createEffect(() => {
        if(!getIsAuthed()){
            return;
        }
        fetch("http://localhost:8080/api/gaps", {
            headers: {
                "Authorization": `bearer ${getToken()}`
            }
        }).then(response => {
            if(response.status === 200){
                return response.json();
            }
            throw response.json();
        }).then(result => {
            let g = [];
            for(let gap of result) {
                g.push({id: gap["gap_in_education"], value: gap["gap_in_education"]})
            }
            gaps[1](g);
        }).catch(error => {
            console.log(error);
            alert("There was an error in loading Gap in Education");
        })
    })
    

    const submitHandler = async (event) => {
        event.preventDefault();
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
            <Field  
                name="University-Id" 
                type="text" 
                data={data} 
                keyname="id" 
                required="false"
            />
            <NField
                name="Graduation Cgpa"
                step="0.01"
                min="0"
                max="10"
                data={data}
                keyname="ug_cgpa"
                required="false"
            />
            <div class="d-flex justify-content-around">
                <CheckList
                    class="col-md-3" 
                    name="Graduation Start Year" 
                    data={data} 
                    keyname="ug_startyear"  
                    list={startyear[0]}
                />
                <CheckList 
                    class="col-md-3" 
                    name="Graduation End Year" 
                    data={data} 
                    keyname="ug_endyear"
                    list={endyear[0]}
                />
                <CheckList
                    name="Graduation Qualification"
                    data={data}
                    list={getQualifications}
                    keyname="ug_qualification_id"
                    required="false"
                />
                <CheckList
                    name="Graduation Specialization"
                    data={data}
                    list={getSpecializations}
                    keyname="ug_specialization_id"
                    required="false"
                />
            </div>
            <Field 
                class="col-md-3" 
                name="Firstname" 
                type="text" 
                data={data}
                keyname="firstname"
                required="false"
            />
            <Field 
                class="col-md-3" 
                name="Lastname" 
                type="text" 
                data={data}
                keyname="lastname"
                required="false"
            />
            <Field 
                class="col-md-3" 
                name="Email" 
                type="email" 
                data={data}
                keyname="email"
                required="false"
            />
            <Field 
                class="col-md-3" 
                name="Phone" 
                type="tel" 
                data={data}
                keyname="phone"
                required="false"
            />
            <div class="d-flex justify-content-around">
                <CheckList 
                    name="Gender"
                    keyname="gender" 
                    data={data}
                    list={getGenders} 
                    required="false"
                />
                <CheckList 
                    name="Category" 
                    keyname="category"
                    data={data}
                    list={getCategories}
                    required="false" 
                />
                <CheckList
                    name="Gap In Education"
                    keyname="gap_in_education"
                    data={data}
                    list={gaps[0]}
                />
                <CheckList
                    name="Number of Backlogs"
                    keyname="no_of_backlogs"
                    data={data}
                    list={backlogs[0]}
                />
            </div>
            <Field 
                name="Father-Name" 
                type="text" 
                data={data}
                keyname="father_name"
                required="false"
            />
            <Field 
                name="Mother-Name" 
                type="text" 
                data={data}
                keyname="mother_name"   
                required="false"  
            />
            <div class="d-flex justify-content-around">
                <CheckList
                    name="Father-Occupation"
                    list={getFatherOccupations}
                    data={data}
                    keyname="father_occupation_id"
                    required="false"
                />
                <CheckList
                    name="Mother-Occupation"
                    list={getMotherOccupations}
                    data={data}
                    keyname="mother_occupation_id"
                    required="false"
                />
                <CheckList 
                    name="Campuses"
                    data={data}
                    keyname="campus_id"
                    list={getCampuses}
                />
                
            </div>
            <NField
                name="Tenth Percentage"
                step="0.01"
                min="33"
                max="100"
                data={data}
                keyname="tenth_percentage"
                required="false"
            />
            <NField
                name="Twelfth Percentage"
                step="0.01"
                min="0"
                max="100"
                data={data}
                keyname="twelfth_percentage"
                required="false"
            />
            <div class="d-flex justify-content-around">
                <CheckList 
                    name="Tenth-Board"
                    list={getBoards}
                    data={data}
                    keyname="tenth_board_id" 
                    required="false"
                />
                <CheckList 
                    name="Tenth Year" 
                    list={tenthYears[0]}
                    data={data}
                    keyname="tenth_year"
                    required="false"
                />
                <CheckList
                    name="Twelfth-Board" 
                    list={getBoards} 
                    data={data}
                    keyname="twelfth_board_id"
                    required="false"
                />
                <CheckList 
                    name="Twelfth Year" 
                    list={twelfthYears[0]} 
                    data={data}
                    keyname="twelfth_year"
                    required="false"
                />
            </div>
            <Field 
                class="col-md-4" 
                name="City" 
                type="text" 
                data={data}
                keyname="city"
                required="false"
            />
            <Field 
                class="col-md-4" 
                name="State" 
                type="text" 
                data={data}
                keyname="state"
                required="false"
            />
            <Field 
                class="col-md-4" 
                name="PinCode" 
                type="text" 
                data={data}
                keyname="zip"
                required="false"
            />
            <Field 
                class="col-md-4" 
                name="NCS Id" 
                type="text" 
                data={data}
                keyname="ncs_id"
                required="false"
            />
            <Field 
                class="col-md-4" 
                name="Linkdin Id" 
                type="text" 
                data={data}
                keyname="linkdin_id"
                required="false"
            />
            <Field 
                class="col-md-4" 
                name="CV Link" 
                type="text" 
                data={data}
                keyname="cv_link"
                required="false"
            />
            <Checkbox 
                name="Interested In Placement" 
                keyname="interested_in_placement"  
                data={data}
            />
            <div class="col-12 d-flex justify-content-center mb-3">
                <button type="submit" class="btn btn-success">Search</button>
            </div>
        </form>
    );
}

export default Filter;
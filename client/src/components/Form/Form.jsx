import { useNavigate } from "solid-app-router";

import { Select, Field, NField, Checkbox } from "./FormFields";
import { Store } from "../../utils";
import { createSignal, Show } from "solid-js";
function Form() {
    const currentYear = new Date().getFullYear();
    const navigate = useNavigate();
    const {
		getBoards, 
		getCategories, 
		getGenders, 
		getFatherOccupations, 
		getMotherOccupations, 
		getQualifications, 
		getSpecializations,
        getCampuses,
		setDisplay,
	} = Store;
    setDisplay(false);
    let personalData = {};
    let familyData = {};
    let pgData = {};
    let data = {};
    const [getShowUg, setShowUg] = createSignal(false);

    const submitHandler = (event) => {
        event.preventDefault();
        console.log(data);
        if(getShowUg()){
            fetch("http://localhost:8080/api/pg", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(pgData)
            }).then(response => {
                return response.json();
            }).then(result => {
                if(!isNaN(result.id)){
                    data["pg_id"] = result.id;
                }
            }).catch(error => {
                console.log(error);
            });
        }else{
            data["pg_id"] = null;
        }

        fetch("http://localhost:8080/api/personal", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(personalData)
        }).then(response => {
            return response.json();
        }).then(result => {
            console.log(result);
            if(!isNaN(result.id)){
                data['personal_id'] = result.id;
                return fetch("http://localhost:8080/api/family", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(familyData)
                });
            }
        })
        .then(response => {
            return response.json();
        })
        .then(result => {
            if(!isNaN(result.id)){
                data['family_id'] = result.id;
                return fetch("http://localhost:8080/api/student", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(data)
                });
            }
        })
        .then(response => {
            return response.json();
        }).then(result => {
            alert("You Are Registered");
            navigate("/");
        })
        .catch(error => {
            alert("Please Check that Email, Phone Number and University are never used before");
            console.log(error);
        })


        
    }

    return (
        <form class="row g-3" onSubmit={submitHandler} style={{margin: "60px 0px"}}>
            <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                Personal Details
            </div>
            <Field name="Firstname" type="text" data={personalData} keyname="firstname" />
            <Field name="Lastname" type="text" data={personalData} keyname="lastname" />
            <Field name="Email" type="email" data={personalData} keyname="email" />
            <Field name="Phone" type="tel" data={personalData} keyname="phone" />
            <Select class="col-md-4" name="Gender" data={personalData} list={getGenders} keyname="gender_id"/>
            <Field class="col-md-4" name="Date-of-Birth" type="date" data={personalData} keyname="dob" />
            <Select class="col-md-4" name="Category" data={personalData} list={getCategories} keyname="category_id" />

            <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                Family Details
            </div>
            <Field name="Father-Name" type="text" data={familyData} keyname="father_name" />
            <Select
                name="Father-Occupation"
                data={familyData}
                list={getFatherOccupations}
                keyname="father_occupation_id"
            />
            <Field name="Mother-Name" type="text" data={familyData} keyname="mother_name" />
            <Select
                name="Mother-Occupation"
                data={familyData}
                list={getMotherOccupations}
                keyname="mother_occupation_id"
            />

            <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                Address
            </div>
            <Field class="col-md=12" name="Address(without city)" type="text" data={familyData} keyname="address" />
            <Field class="col-md-4" name="City" type="text" data={familyData} keyname="city" />
            <Field class="col-md-4" name="State" type="text" data={familyData} keyname="state" />
            <Field class="col-md-4" name="PinCode" type="text" data={familyData} keyname="zip" />
            <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                Study Details
            </div>
            <Select class="col-md-4" name="Tenth-Board" data={familyData} list={getBoards} keyname="tenth_board_id" />
            <NField
                class="col-md-4"
                name="Tenth Percentage"
                step="0.01"
                min="0"
                max="100"
                data={familyData}
                keyname="tenth_percentage"
            />
            <NField class="col-md-4" name="Tenth Year" step="1" min={currentYear - 10} max={currentYear} data={familyData} keyname="tenth_year" />
            <Select class="col-md-4" name="Twelfth-Board" list={getBoards} data={familyData} keyname="twelfth_board_id" />
            <NField
                class="col-md-4" 
                name="Twelfth Percentage"
                step="0.01"
                min="0"
                max="100"
                data={familyData}
                keyname="twelfth_percentage"
            />
            <NField class="col-md-4" name="Twelfth Year" step="1" min={currentYear - 10} max={currentYear} data={familyData} keyname="twelfth_year" />

            <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                Graduation Details
            </div>
            <Field name="University-Id" type="text" data={data} keyname="id" />
            <NField
                name="Graduation Cgpa"
                step="0.01"
                min="0"
                max="100"
                data={personalData}
                keyname="ug_cgpa"
            />
            <Select
                name="Graduation Qualification"
                list={getQualifications}
                data={personalData}
                keyname="ug_qualification_id"
            />
            <Select
                name="Graduation Specialization"
                list={getSpecializations}
                data={personalData}
                keyname="ug_specialization_id"
            />
            <NField name="Graduation Start Year" step="1" min={currentYear - 10} max={currentYear + 4} data={personalData} keyname="ug_startyear" />
            <NField name="Graduation End Year"  step="1" min={currentYear - 10} max={currentYear + 4}  data={personalData} keyname="ug_endyear" />

            <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                Other Details
            </div>
            <Field name="Jee Rank" type="number" data={data} keyname="jee_rank" />
            <Select name="Campus" list={getCampuses} data={data} keyname="campus_id" />
            <Field name="Number of backlogs" type="number" data={data} keyname="no_of_backlogs" />
            <Field name="Reason For Backlogs" type="text" data={data} keyname="reason_for_backlogs" />
            <Field name="Gap in Education (In Years)" type="number" data={data} keyname="gap_in_education" />
            <Field name="Reason For Gap" data={data} type="text" keyname="reason_for_gap" />
            <Field name="NCS Id" data={data} type="text" keyname="ncs_id" />
            <Field name="Linkdin URL" data={data} type="text" keyname="linkdin_id" />
            <Field name="CV Link" data={data} keyname="cv_link" type="text" />
            <Checkbox name="Interested In Placement" data={data} keyname="interested_in_placement" />
            <div class="form-check m-3">
                <input class="form-check-input" type="checkbox" onChange={() => setShowUg(v => !v)} id="flexCheckDefault" />
                <label class="form-check-label" for="flexCheckDefault">
                    Enter Post Graduation Data
                </label>
            </div>
            <Show
                when={getShowUg()}
            >
                <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
                    Post Graduation Details
                </div>
                <NField
                    name="Post Graduation Cgpa"
                    step="0.01"
                    min="0"
                    max="100"
                    data={pgData}
                    keyname="cgpa"
                />
                <Select
                    name="Post Graduation Qualification"
                    list={getQualifications}
                    data={pgData}
                    keyname="qualification_id"
                />
                <Select
                    name="Post Graduation Specialization"
                    list={getSpecializations}
                    data={pgData}
                    keyname="specialization_id"
                />
                <NField name="Post Graduation Start Year" data={pgData} keyname="startyear"  step="1" min={currentYear - 10} max={currentYear + 4}  />
                <NField name="Post Graduation End Year" data={pgData} keyname="endyear" step="1" min={currentYear - 10} max={currentYear + 4}  />
            </Show>
            <div class="col-12 d-flex justify-content-center">
                <button class="btn btn-primary" type="submit">Submit</button>
            </div>
        </form>
    );
}

export default Form;
import { createSignal } from "solid-js";
import { Select, NField } from "../../Helpers";
import { Store } from "../../utils";

import Field from './Field';

function Form() {
	const {
		getBoards, 
		getCategories, 
		getGenders, 
		getFatherOccupations, 
		getMotherOccupations, 
		getQualifications, 
		getSpecializations,
		setDisplay
	} = Store;
	let data = {};
	setDisplay(false);
	const universityId = createSignal(null);
	const firstname = createSignal("");
	const lastname = createSignal("");
	const email = createSignal("");
	const phone = createSignal("");
	const dob = createSignal(new Date().setFullYear(2000).toString);
	const gender = createSignal(0);
	const category = createSignal(0);
	const fatherName = createSignal("");
	const motherName = createSignal("");
	const fatherOccupation = createSignal(0);
	const motherOccupation = createSignal(0);
	const address = createSignal("");
	const city = createSignal("");
	const state = createSignal("");
	const zip = createSignal("");
	const tenthBoard = createSignal(0);
	const tenthPercentage = createSignal(0);
	const tenthYear = createSignal(0);
	const twelfthBoard = createSignal(0);
	const twelfthPercentage = createSignal(0);
	const twelfthYear = createSignal(0);
	const ugQualification = createSignal(0);
	const ugSpecialization = createSignal(0);
	const ugStartYear = createSignal(0);
	const ugEndYear = createSignal(0);
	const ugCgpa = createSignal(0);

	const submitHandler = (event) => {
		event.preventDefault();
		// error in data types and value types
		const personal = {
			firstname: firstname[0](),
			lastname: lastname[0](),
			email: email[0](),
			phone: phone[0](),
			dob: dob[0](),
			gender_id: parseInt(gender[0]()),
			category_id: parseInt(category[0]()),
			ug_qualification_id: parseInt(ugQualification[0]()),
			ug_specialization_id: parseInt(ugSpecialization[0]()),
			ug_startyear: parseInt(ugStartYear[0]().split("-")[0]),
			ug_endyear: parseInt(ugEndYear[0]().split("-")[0]),
			ug_cgpa: parseFloat(ugCgpa[0]()),
		};
		const family = {
			father_name: fatherName[0](),
			mother_name: motherName[0](),
			father_occupation_id: parseInt(fatherOccupation[0]()),
			mother_occupation_id: parseInt(motherOccupation[0]()),
			address: address[0](),
			city: city[0](),
			state: state[0](),
			zip: zip[0](),
			tenth_board_id: parseInt(tenthBoard[0]()),
			tenth_percentage: parseFloat(tenthPercentage[0]()),
			tenth_year: parseInt(tenthYear[0]().split("-")[0]),
			twelfth_board_id: parseInt(twelfthBoard[0]()),
			twelfth_percentage: parseFloat(twelfthPercentage[0]()),
			twelfth_year: parseInt(twelfthYear[0]().split("-")[0]),
		};
		const data = {
			id: universityId[0](),
			pg_id: null,
		};
		fetch("http://localhost:8080/api/personal", {
			method: "POST",
			headers: {
			"Content-Type": "application/json",
			},
			body: JSON.stringify(personal),
		}).then((data) => {
			console.log(data);
			return data.json();
		}).then((result) => {
			console.log(result);
			data.personal_id = parseInt(result.id);
			return fetch("http://localhost:8080/api/family", {
				method: "POST",
				headers: {
				"Content-Type": "application/json",
				},
				body: JSON.stringify(family),
			});
		}).then((data) => {
			return data.json();
		}).then((result) => {
			data.family_id = parseInt(result.id);
			return fetch("http://localhost:8080/api/student", {
				method: "POST",
				headers: {
				"Content-Type": "application/json",
				},
				body: JSON.stringify(data),
			});
		}).then((data) => {
			return data.json();
		}).then((result) => {
			console.log(result);
		}).catch((error) => {
			console.log(error);
		});
		console.log(data);
	};

	
	return (
	<form class="row g-3" onSubmit={submitHandler} style={{margin: "60px 0px"}}>
		<div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
		Personal Details
		</div>
		<Field name="Firstname" type="text" model={firstname} />
		<Field name="Lastname" type="text" model={lastname} />
		<Field name="Email" type="email" model={email} />
		<Field name="Phone" type="tel" model={phone} />
		<Select class="col-md-4" name="Gender" model={gender} list={getGenders} />
		<Field class="col-md-4" name="Date-of-Birth" type="date" model={dob} />
		<Select class="col-md-4" name="Category" model={category} list={getCategories} />

		<div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
		Family Details
		</div>
		<Field name="Father-Name" type="text" model={fatherName} />
		<Select
		name="Father-Occupation"
		model={fatherOccupation}
		list={getFatherOccupations}
		/>
		<Field name="Mother-Name" type="text" model={motherName} />
		<Select
		name="Mother-Occupation"
		model={motherOccupation}
		list={getMotherOccupations}
		/>

		<div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
		Address
		</div>
		<Field class="col-md=12" name="Address(without city)" type="text" model={address} />
		<Field class="col-md-4" name="City" type="text" model={city} />
		<Field class="col-md-4" name="State" type="text" model={state} />
		<Field class="col-md-4" name="PinCode" type="text" model={zip} />
		<div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
		Study Details
		</div>
		<Select class="col-md-4" name="Tenth-Board" model={tenthBoard} list={getBoards} />
		<NField
		class="col-md-4"
		name="Tenth Percentage"
		step="0.01"
		min="0"
		max="100"
		model={tenthPercentage}
		/>
		<Field class="col-md-4" name="Tenth Year" type="month" model={tenthYear} />

		{/* <div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
		Twelfth Details
		</div> */}
		<Select class="col-md-4" name="Twelfth-Board" model={twelfthBoard} list={getBoards} />
		<NField
		class="col-md-4" 
		name="Twelfth Percentage"
		step="0.01"
		min="0"
		max="100"
		model={twelfthPercentage}
		/>
		<Field class="col-md-4" name="Twelfth Year" type="month" model={twelfthYear} />

		<div class="p-3 bg-info bg-opacity-10 border border-info border-start-0 rounded-end">
		Graduation Details
		</div>
		<Field name="University-Id" type="text" model={universityId} />
		<Field
		name="Graduation Cgpa"
		step="0.01"
		min="0"
		max="100"
		model={ugCgpa}
		/>
		<Select
		name="Graduation Qualification"
		model={ugQualification}
		list={getQualifications}
		/>
		<Select
		name="Graduation Specialization"
		model={ugSpecialization}
		list={getSpecializations}
		/>
		<Field name="Graduation Start Year" type="month" model={ugStartYear} />
		<Field name="Graduation End Year" type="month" model={ugEndYear} />
		<div class="col-12 d-flex justify-content-center">
		<button class="btn btn-primary" type="submit">Submit</button>
		</div>
	</form>
	);
	}
export default Form;

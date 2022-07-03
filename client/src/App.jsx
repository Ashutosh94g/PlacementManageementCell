import { Route, Routes } from "solid-app-router";


import { Header, Footer } from "./components/Layout";
import Form from "./components/Form/Form";

import { Login, Register } from "./components/Auth";
import { Filter, Table } from "./components/Filter";
import AddCategory from "./components/Category/AddCategory";

function App() {
  return (
    <>
      <Header />
      <Routes>
        <Route path="/add-category" element={<AddCategory />} />
        <Route path="/filter" element={<Filter />} />
        <Route path="/table" element={<Table />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="/" element={<Form />} />
      </Routes>
      <Footer />
    </>
  );
}

export default App;

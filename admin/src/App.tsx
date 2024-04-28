import { Admin } from "@/components/admin";
import simpleRestProvider from 'ra-data-simple-rest';

import { Dashboard } from "./components/dashboard/Dashboard";



const dataProvider = simpleRestProvider('/api');

function App() {
    return (
        <Admin
            dataProvider={dataProvider}
            dashboard={Dashboard}
        >
            
        </Admin>
    );
}
export default App;
import { Bookmark } from "lucide-react";
import { Create, DataTable, Edit, List, SimpleForm, TextInput } from "@/components/admin";
import { ResourceProps, required } from "ra-core";

export const GroupCreate = () => (
    <Create redirect="list">
        <SimpleForm>
            <TextInput source="name" label="Name" validate={required()} />
            <TextInput source="description" />
            <TextInput source="language" />
            <TextInput source="name" />
            <TextInput source="website" />
            
        </SimpleForm>
    </Create>
);

export const GroupEdit = () => (
    <Edit>
        <div className="flex flex-col lg:flex-row items-start justify-between">
            <SimpleForm>
                <TextInput source="description" />
                <TextInput source="language" />
                <TextInput source="name" />
                <TextInput source="website" />
                
            </SimpleForm>
        </div>
    </Edit>
);

export const GroupList = () => (
    <List perPage={50}>
        <DataTable>
            <DataTable.Col source="description" />
            <DataTable.Col source="language" />
            <DataTable.Col source="name" />
            <DataTable.Col source="website" />
            
        </DataTable>
    </List>
);

export const Groups: ResourceProps = {
    name: "groups",
    list: GroupList,
    edit: GroupEdit,
    create: GroupCreate,
    recordRepresentation: "name",
    icon: Bookmark,
};

export default Groups;
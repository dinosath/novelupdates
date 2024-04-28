import { Bookmark } from "lucide-react";
import { Create, DataTable, Edit, List, SimpleForm, TextInput } from "@/components/admin";
import { ResourceProps, required } from "ra-core";

export const NovelTypeCreate = () => (
    <Create redirect="list">
        <SimpleForm>
            <TextInput source="name" label="Name" validate={required()} />
            <TextInput source="description" />
            <TextInput source="name" />
            
        </SimpleForm>
    </Create>
);

export const NovelTypeEdit = () => (
    <Edit>
        <div className="flex flex-col lg:flex-row items-start justify-between">
            <SimpleForm>
                <TextInput source="description" />
                <TextInput source="name" />
                
            </SimpleForm>
        </div>
    </Edit>
);

export const NovelTypeList = () => (
    <List perPage={50}>
        <DataTable>
            <DataTable.Col source="description" />
            <DataTable.Col source="name" />
            
        </DataTable>
    </List>
);

export const NovelTypes: ResourceProps = {
    name: "novel-types",
    list: NovelTypeList,
    edit: NovelTypeEdit,
    create: NovelTypeCreate,
    recordRepresentation: "name",
    icon: Bookmark,
};

export default NovelTypes;
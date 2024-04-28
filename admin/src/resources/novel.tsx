import { Bookmark } from "lucide-react";
import { Create, DataTable, Edit, List, SimpleForm, ReferenceInput, TextInput } from "@/components/admin";
import { ResourceProps, required } from "ra-core";

export const NovelCreate = () => (
    <Create redirect="list">
        <SimpleForm>
            <TextInput source="name" label="Name" validate={required()} />
            <TextInput source="defaultName" />
            <TextInput source="description" />
            <TextInput source="genres" />
            <TextInput source="originalLanguage" />
            <TextInput source="sources" />
            <TextInput source="statusOrigin" />
            <TextInput source="tags" />
            <ReferenceInput source="type.id" reference="types"/>
            
        </SimpleForm>
    </Create>
);

export const NovelEdit = () => (
    <Edit>
        <div className="flex flex-col lg:flex-row items-start justify-between">
            <SimpleForm>
                <TextInput source="defaultName" />
                <TextInput source="description" />
                <TextInput source="genres" />
                <TextInput source="originalLanguage" />
                <TextInput source="sources" />
                <TextInput source="statusOrigin" />
                <TextInput source="tags" />
                <ReferenceInput source="type.id" reference="types"/>
                
            </SimpleForm>
        </div>
    </Edit>
);

export const NovelList = () => (
    <List perPage={50}>
        <DataTable>
            <DataTable.Col source="defaultName" />
            <DataTable.Col source="description" />
            <DataTable.Col source="genres" />
            <DataTable.Col source="originalLanguage" />
            <DataTable.Col source="sources" />
            <DataTable.Col source="statusOrigin" />
            <DataTable.Col source="tags" />
            <DataTable.Col source="type.id" />
            
        </DataTable>
    </List>
);

export const Novels: ResourceProps = {
    name: "novels",
    list: NovelList,
    edit: NovelEdit,
    create: NovelCreate,
    recordRepresentation: "name",
    icon: Bookmark,
};

export default Novels;
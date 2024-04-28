import { Bookmark } from "lucide-react";
import { Create, DataTable, Edit, List, SimpleForm, ReferenceInput, TextInput } from "@/components/admin";
import { ResourceProps, required } from "ra-core";

export const SourceCreate = () => (
    <Create redirect="list">
        <SimpleForm>
            <TextInput source="name" label="Name" validate={required()} />
            <TextInput source="chapters" />
            <ReferenceInput source="group.id" reference="groups"/>
            <TextInput source="language" />
            <TextInput source="name" />
            <ReferenceInput source="novel.id" reference="novels"/>
            <TextInput source="url" />
            
        </SimpleForm>
    </Create>
);

export const SourceEdit = () => (
    <Edit>
        <div className="flex flex-col lg:flex-row items-start justify-between">
            <SimpleForm>
                <TextInput source="chapters" />
                <ReferenceInput source="group.id" reference="groups"/>
                <TextInput source="language" />
                <TextInput source="name" />
                <ReferenceInput source="novel.id" reference="novels"/>
                <TextInput source="url" />
                
            </SimpleForm>
        </div>
    </Edit>
);

export const SourceList = () => (
    <List perPage={50}>
        <DataTable>
            <DataTable.Col source="chapters" />
            <DataTable.Col source="group.id" />
            <DataTable.Col source="language" />
            <DataTable.Col source="name" />
            <DataTable.Col source="novel.id" />
            <DataTable.Col source="url" />
            
        </DataTable>
    </List>
);

export const Sources: ResourceProps = {
    name: "sources",
    list: SourceList,
    edit: SourceEdit,
    create: SourceCreate,
    recordRepresentation: "name",
    icon: Bookmark,
};

export default Sources;
import { Bookmark } from "lucide-react";
import { Create, DataTable, Edit, List, SimpleForm, ReferenceInput, TextInput } from "@/components/admin";
import { ResourceProps, required } from "ra-core";

export const ChapterCreate = () => (
    <Create redirect="list">
        <SimpleForm>
            <TextInput source="name" label="Name" validate={required()} />
            <TextInput source="contentUrl" />
            <TextInput source="language" />
            <TextInput source="number" />
            <TextInput source="releaseDate" />
            <ReferenceInput source="source.id" reference="sources"/>
            <TextInput source="title" />
            
        </SimpleForm>
    </Create>
);

export const ChapterEdit = () => (
    <Edit>
        <div className="flex flex-col lg:flex-row items-start justify-between">
            <SimpleForm>
                <TextInput source="contentUrl" />
                <TextInput source="language" />
                <TextInput source="number" />
                <TextInput source="releaseDate" />
                <ReferenceInput source="source.id" reference="sources"/>
                <TextInput source="title" />
                
            </SimpleForm>
        </div>
    </Edit>
);

export const ChapterList = () => (
    <List perPage={50}>
        <DataTable>
            <DataTable.Col source="contentUrl" />
            <DataTable.Col source="language" />
            <DataTable.Col source="number" />
            <DataTable.Col source="releaseDate" />
            <DataTable.Col source="source.id" />
            <DataTable.Col source="title" />
            
        </DataTable>
    </List>
);

export const Chapters: ResourceProps = {
    name: "chapters",
    list: ChapterList,
    edit: ChapterEdit,
    create: ChapterCreate,
    recordRepresentation: "name",
    icon: Bookmark,
};

export default Chapters;
import { Bookmark } from "lucide-react";
import { Create, DataTable, Edit, List, SimpleForm, TextInput } from "@/components/admin";
import { ResourceProps, required } from "ra-core";

export const TagCreate = () => (
    <Create redirect="list">
        <SimpleForm>
            <TextInput source="name" label="Name" validate={required()} />
            <TextInput source="name" />
            <TextInput source="slug" />
            
        </SimpleForm>
    </Create>
);

export const TagEdit = () => (
    <Edit>
        <div className="flex flex-col lg:flex-row items-start justify-between">
            <SimpleForm>
                <TextInput source="name" />
                <TextInput source="slug" />
                
            </SimpleForm>
        </div>
    </Edit>
);

export const TagList = () => (
    <List perPage={50}>
        <DataTable>
            <DataTable.Col source="name" />
            <DataTable.Col source="slug" />
            
        </DataTable>
    </List>
);

export const Tags: ResourceProps = {
    name: "tags",
    list: TagList,
    edit: TagEdit,
    create: TagCreate,
    recordRepresentation: "name",
    icon: Bookmark,
};

export default Tags;
import { Translate } from "ra-core";
import { Breadcrumb, BreadcrumbPage } from "@/components/admin";

import Welcome from "./Welcome";

export const Dashboard = () => {


  return (
      <>
        <Breadcrumb>
          <BreadcrumbPage>
            <Translate i18nKey="ra.page.dashboard">Home</Translate>
          </BreadcrumbPage>
        </Breadcrumb>
        <Welcome />
        <div className="flex flex-col md:flex-row gap-4 mb-4">
          <div className="flex flex-col gap-4 md:basis-1/2">
            <div className="flex flex-col md:flex-row gap-4">
            </div>
            <div>
            </div>
            <div>
            </div>
          </div>
          <div className="md:basis-1/2">
            <div className="flex flex-col md:flex-row gap-4">
            </div>
          </div>
        </div>
      </>
  );
};

export default Dashboard;

import projectFieldsConfig from "../../fields/config/project/projectFieldsConfig";
import testbedFieldsConfig from "../../fields/config/project/testbedFieldsConfig";
import { Button, Field, Operation, Row } from "./types";
import { BENCHER_API_URL, parentPath, addPath, viewPath } from "./util";

const testbedsConfig = {
  [Operation.LIST]: {
    operation: Operation.LIST,
    header: {
      title: "Testbeds",
      buttons: [
        {
          kind: Button.ADD,
          path: (pathname) => {
            return addPath(pathname);
          },
        },
        { kind: Button.REFRESH },
      ],
    },
    table: {
      url: (path_params) => {
        return `${BENCHER_API_URL}/v0/projects/${path_params?.project_slug}/testbeds`;
      },
      row: {
        key: "name",
        items: [
          {
            kind: Row.TEXT,
            key: "slug",
          },
          {},
          {},
          {},
        ],
        path: (pathname, datum) => {
          return viewPath(pathname, datum);
        },
      },
    },
  },
  [Operation.ADD]: {
    operation: Operation.ADD,
    header: {
      title: "Add Testbed",
      path: (pathname) => {
        return parentPath(pathname);
      },
    },
    form: {
      url: `${BENCHER_API_URL}/v0/testbeds`,
      fields: [
        {
          kind: Field.FIXED,
          key: "project",
          label: true,
          value: "",
          valid: null,
          validate: true,
          nullify: false,
          clear: false,
          config: projectFieldsConfig.slug,
        },
        {
          kind: Field.INPUT,
          key: "name",
          label: true,
          value: "",
          valid: null,
          validate: true,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.name,
        },
        {
          kind: Field.INPUT,
          key: "os_name",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.os_name,
        },
        {
          kind: Field.INPUT,
          key: "os_version",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.os_version,
        },
        {
          kind: Field.INPUT,
          key: "runtime_name",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.runtime_name,
        },
        {
          kind: Field.INPUT,
          key: "runtime_version",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.runtime_version,
        },
        {
          kind: Field.INPUT,
          key: "cpu",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.cpu,
        },
        {
          kind: Field.INPUT,
          key: "ram",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.ram,
        },
        {
          kind: Field.INPUT,
          key: "disk",
          label: true,
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: testbedFieldsConfig.disk,
        },
      ],
      path: (pathname) => {
        return parentPath(pathname);
      },
    },
  },
  [Operation.VIEW]: {
    operation: Operation.VIEW,
    header: {
      key: "name",
      path: (pathname) => {
        return parentPath(pathname);
      },
    },
    deck: {
      url: (path_params) => {
        return `${BENCHER_API_URL}/v0/projects/${path_params?.project_slug}/testbeds/${path_params?.testbed_slug}`;
      },
      cards: [
        {
          field: "Testbed Name",
          key: "name",
        },
        {
          field: "Testbed Slug",
          key: "slug",
        },
      ],
      buttons: false,
    },
  },
};

export default testbedsConfig;
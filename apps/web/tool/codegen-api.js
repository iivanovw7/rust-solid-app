const { generateApi } = require("swagger-typescript-api");
const path = require("path");

(async () => {
    	await generateApi({
		output: path.resolve(__dirname, "../src/shared/api/http/__generated__"),
		url: "http://127.0.0.1:8088/api/api-docs/openapi.json",
		templates: path.resolve(__dirname, "./templates"),
		httpClientType: "axios",
		defaultResponseAsSuccess: false,
		generateRouteTypes: false,
		generateResponses: true,
		toJS: false,
		defaultResponseType: "void",
		singleHttpClient: true,
		enumNamesAsValues: true,
		moduleNameFirstTag: true,
		generateUnionEnums: true,
        modular: true,
        hooks: {
            onCreateRouteName: (routeNameInfo) => ({
                original: routeNameInfo.original.replace(/\d+$/, ""),
                usage: routeNameInfo.usage.replace(/\d+$/, ""),
                duplicate: false,
            }),
            onFormatTypeName: (typeName) => `Query${typeName}`,
            onParseSchema: (_, parsedSchema) => {
                if (Array.isArray(parsedSchema.content)) {
                    parsedSchema.content.forEach((parsedType) => {
                        if (!parsedType.isRequired) {
                            parsedType.field = `${parsedType.name}?: ${parsedType.value} | null`;
                        }
                    });
                }

                return parsedSchema;
            },
        },
	})
})();


initSidebarItems({"enum":[["ColumnArity","A column’s arity."],["ColumnTypeFamily","Enumeration of column type families."],["DefaultKind","A DefaultValue"],["DescriberErrorKind","Variants of DescriberError."],["ForeignKeyAction","Foreign key action types (for ON DELETE|ON UPDATE)."],["IndexType","The type of an index."],["SQLSortOrder","The sort order of an index."],["SqlIndexAlgorithm",""]],"fn":[["unquote_string",""]],"mod":[["mssql",""],["mysql",""],["postgres","Postgres schema description."],["sqlite","SQLite description."],["walkers","Functions and types for conveniently traversing and querying a SqlSchema."]],"struct":[["Column","A column of a table."],["ColumnId","The identifier for a column in a SqlSchema."],["ColumnType","The type of a column."],["DefaultValue",""],["DescriberError","Description errors."],["Enum","A SQL enum."],["EnumId","The identifier for an enum in a SqlSchema."],["ForeignKey","A foreign key."],["Index","An index of a table."],["IndexColumn",""],["IndexFieldId","The identifier for an Index in a SqlSchema."],["IndexId","The identifier for an Index in a SqlSchema."],["PrimaryKey","The primary key of a table."],["PrimaryKeyColumn",""],["Procedure","A stored procedure (like, the function inside your database)."],["SqlMetadata",""],["SqlSchema","The result of describing a database schema."],["Table","A table found in a schema."],["TableId","The identifier for a table in a SqlSchema. Use it with the indexing syntax: `let table = schema[table_id];`"],["UserDefinedType","A user-defined type. Can map to another type, or be declared as assembly."],["View","An SQL view."]],"trait":[["SqlSchemaDescriberBackend","A database description connector."]],"type":[["DescriberResult","The result type."]]});
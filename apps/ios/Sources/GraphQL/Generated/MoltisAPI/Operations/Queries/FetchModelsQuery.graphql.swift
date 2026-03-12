// @generated
// This file was automatically generated and should not be edited.

@_exported import ApolloAPI
@_spi(Execution) @_spi(Unsafe) import ApolloAPI

extension ClawMasterAPI {
  struct FetchModelsQuery: GraphQLQuery {
    static let operationName: String = "FetchModels"
    static let operationDocument: ApolloAPI.OperationDocument = .init(
      definition: .init(
        #"query FetchModels { models { __typename list { __typename id name provider } } }"#
      ))

    public init() {}

    struct Data: ClawMasterAPI.SelectionSet {
      let __data: DataDict
      init(_dataDict: DataDict) { __data = _dataDict }

      static var __parentType: any ApolloAPI.ParentType { ClawMasterAPI.Objects.QueryRoot }
      static var __selections: [ApolloAPI.Selection] { [
        .field("models", Models.self),
      ] }
      static var __fulfilledFragments: [any ApolloAPI.SelectionSet.Type] { [
        FetchModelsQuery.Data.self
      ] }

      /// Model queries.
      var models: Models { __data["models"] }

      /// Models
      ///
      /// Parent Type: `ModelQuery`
      struct Models: ClawMasterAPI.SelectionSet {
        let __data: DataDict
        init(_dataDict: DataDict) { __data = _dataDict }

        static var __parentType: any ApolloAPI.ParentType { ClawMasterAPI.Objects.ModelQuery }
        static var __selections: [ApolloAPI.Selection] { [
          .field("__typename", String.self),
          .field("list", [List].self),
        ] }
        static var __fulfilledFragments: [any ApolloAPI.SelectionSet.Type] { [
          FetchModelsQuery.Data.Models.self
        ] }

        /// List enabled models.
        var list: [List] { __data["list"] }

        /// Models.List
        ///
        /// Parent Type: `ModelInfo`
        struct List: ClawMasterAPI.SelectionSet {
          let __data: DataDict
          init(_dataDict: DataDict) { __data = _dataDict }

          static var __parentType: any ApolloAPI.ParentType { ClawMasterAPI.Objects.ModelInfo }
          static var __selections: [ApolloAPI.Selection] { [
            .field("__typename", String.self),
            .field("id", String?.self),
            .field("name", String?.self),
            .field("provider", String?.self),
          ] }
          static var __fulfilledFragments: [any ApolloAPI.SelectionSet.Type] { [
            FetchModelsQuery.Data.Models.List.self
          ] }

          var id: String? { __data["id"] }
          var name: String? { __data["name"] }
          var provider: String? { __data["provider"] }
        }
      }
    }
  }

}
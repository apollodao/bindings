use cosmwasm_std::{
    from_binary, to_binary, Querier, QuerierResult, QuerierWrapper, QueryRequest, StdResult,
};

use crate::query::{FullDenomResponse, OsmosisQuery, PoolStateResponse};

/// This is a helper wrapper to easily use our custom queries
pub struct OsmosisQuerier<'a> {
    pub querier: &'a QuerierWrapper<'a, OsmosisQuery>,
}

impl Querier for OsmosisQuerier<'_> {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.querier.raw_query(bin_request)
    }
}

impl<'a> OsmosisQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<OsmosisQuery>) -> Self {
        OsmosisQuerier { querier }
    }

    pub fn query_pool_state(&self, pool_id: u64) -> StdResult<PoolStateResponse> {
        QuerierWrapper::new(self).query(&OsmosisQuery::PoolState { id: pool_id }.into())
    }

    pub fn full_denom(
        &self,
        creator_addr: String,
        subdenom: String,
    ) -> StdResult<FullDenomResponse> {
        let full_denom_query = OsmosisQuery::FullDenom {
            creator_addr,
            subdenom,
        };
        let request: QueryRequest<OsmosisQuery> = OsmosisQuery::into(full_denom_query);
        self.querier.query(&request)
    }
}

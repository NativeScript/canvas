// The guard is evaluated first: see ./querySelectorGuard.
import { restoreDocument } from './querySelectorGuard';
import querySelector from 'query-selector';

restoreDocument();

export default querySelector;
